use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};
use async_trait::async_trait;
use tokio::sync::RwLock;
use crate::channel::Channel;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::payload::Payload;

pub struct PublicChannel {
    name: String,
    connections: HashMap<String, Arc<RwLock<Client<'static>>>>,
}

impl PublicChannel {
    #[inline]
    pub fn new(name: String) -> Self {
        Self {
            name,
            connections: HashMap::with_capacity(32),
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl Channel for PublicChannel {
    #[inline]
    fn verify_signature(&self, _client: &Client<'_>, _payload: &Payload) -> Result<(), FastSocketError> {
        Ok(())
    }

    #[inline]
    async fn save_connection(&mut self, client: Client<'_>) -> Result<(), FastSocketError> {
        let socket_id = client.get_socket_id().to_string();
        Log::debug(&format!("New connection from {}", socket_id));

        // Convert client to 'static lifetime using Arc<RwLock>
        let client = unsafe { std::mem::transmute::<Client<'_>, Client<'static>>(client) };
        self.connections.insert(socket_id, Arc::new(RwLock::new(client)));
        Ok(())
    }

    #[inline]
    fn has_connection(&self) -> bool {
        !self.connections.is_empty()
    }

    #[inline]
    fn get_subscribers(&self) -> &HashMap<String, Arc<RwLock<Client<'static>>>> {
        &self.connections
    }

    #[inline]
    async fn subscribe(&mut self, _client: &Client<'_>, _payload: &Payload) -> Result<(), FastSocketError> {
        Ok(())
    }

    #[inline]
    async fn unsubscribe(&mut self, socket_id: &str) -> Result<(), FastSocketError> {
        self.connections.remove(socket_id);
        Ok(())
    }

    #[inline]
    fn get_clients_count(&self) -> u64 {
        self.connections.len() as u64
    }

    async fn broadcast(&mut self, payload: &Payload) -> Result<(), FastSocketError> {
        for client in self.connections.values() {
            let mut client = client.write().await;
            client.send(payload).await?;
        }
        Ok(())
    }

    async fn broadcast_to_others(&mut self, client: &Client<'_>, payload: &Payload) -> Result<(), FastSocketError> {
        let socket_id = client.get_socket_id();
        self.broadcast_to_everyone_except(socket_id, payload).await
    }

    async fn broadcast_to_everyone_except(&mut self, socket_id: &str, payload: &Payload) -> Result<(), FastSocketError> {
        for (id, client) in self.connections.iter() {
            if id != socket_id {
                let mut client = client.write().await;
                client.send(payload).await?;
            }
        }
        Ok(())
    }

    #[inline]
    fn to_array(&self) -> Value {
        json!({
            "occupied": self.has_connection(),
            "subscription_count": self.get_clients_count(),
        })
    }
}

// Safety: PublicChannel can be safely shared between threads
unsafe impl Send for PublicChannel {}
unsafe impl Sync for PublicChannel {}
