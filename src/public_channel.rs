use crate::channel::Channel;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::payload::Payload;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PublicChannel {
    name: String,
    connections: HashMap<String, Arc<Client>>,
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
    fn verify_signature(&self, _client: Arc<Client>, _payload: &Payload) -> Result<(), FastSocketError> {
        Ok(())
    }

    #[inline]
    async fn save_connection(&mut self, client: Arc<Client>) -> Result<(), FastSocketError> {
        let socket_id = client.get_socket_id().to_string();
        Log::debug(&format!("New connection from {}", socket_id));

        self.connections.insert(socket_id, client);
        Ok(())
    }

    #[inline]
    fn has_connection(&self) -> bool {
        !self.connections.is_empty()
    }

    #[inline]
    fn get_subscribers(&self) -> &HashMap<String, Arc<Client>> {
        &self.connections
    }

    #[inline]
    async fn subscribe(&mut self, _client: Arc<Client>, _payload: &Payload) -> Result<(), FastSocketError> {
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
            client.socket().lock().await.send(payload).await?;
        }
        Ok(())
    }

    async fn broadcast_to_others(&mut self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError> {
        let socket_id = client.get_socket_id();
        self.broadcast_to_everyone_except(socket_id, payload).await
    }

    async fn broadcast_to_everyone_except(&mut self, socket_id: &str, payload: &Payload) -> Result<(), FastSocketError> {
        for (id, client) in self.connections.iter() {
            if id != socket_id {
                client.socket().lock().await.send(payload).await?;
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
