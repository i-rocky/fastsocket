use crate::client::Client;
use crate::errors::FastSocketError;
use crate::payload::Payload;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::logger::Log;

#[async_trait]
pub trait Channel: Send + Sync {
    fn get_name(&self) -> &str;
    fn get_connections(&self) -> &RwLock<HashMap<String, Arc<Client>>>;

    #[inline]
    async fn verify_signature(&self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError> {
        Ok(())
    }

    #[inline]
    async fn save_connection(&mut self, client: Arc<Client>) -> Result<(), FastSocketError> {
        let socket_id = client.get_socket_id().to_string();
        Log::debug(&format!("Saving new connection from {}", socket_id));

        let mut write_guard = self.get_connections().write().await;
        write_guard.insert(socket_id, client);
        drop(write_guard);

        Log::debug("Saved new connection from");

        Ok(())
    }

    #[inline]
    async fn has_connection(&self) -> bool {
        let read_guard = self.get_connections().read().await;
        let has_connection = !read_guard.is_empty();
        drop(read_guard);
        has_connection
    }

    #[inline]
    async fn get_subscribers(&self) -> HashMap<String, Arc<Client>> {
        let read_guard = self.get_connections().read().await;
        let subscribers = read_guard.clone();
        drop(read_guard);
        subscribers
    }



    #[inline]
    async fn subscribe(&mut self, _client: Arc<Client>, _payload: &Payload) -> Result<(), FastSocketError> {
        Log::debug("Subscribing");
        self.save_connection(_client.clone()).await?;

        Log::debug("Creating subscription succeeded payload");
        let payload = Payload::builder()
            .event("pusher_internal:subscription_succeeded")
            .channel(self.get_name())
            .build();

        if payload.is_err() {
            return Err(FastSocketError::FailedToSendPayloadError);
        }

        Log::debug("Sending subscription succeeded");
        let socket = _client.socket();
        let mut guard = socket.lock().await;
        guard.send(&payload?).await?;

        Log::debug("Subscription succeeded sent");

        Ok(())
    }

    #[inline]
    async fn unsubscribe(&mut self, socket_id: &str) -> Result<(), FastSocketError> {
        Log::debug(&format!("Removing connection: {}", socket_id));
        let mut write_guard = self.get_connections().write().await;
        write_guard.remove(socket_id);
        drop(write_guard);
        Log::debug(&format!("Removed connection: {}", socket_id));
        Ok(())
    }

    #[inline]
    async fn get_clients_count(&self) -> u64 {
        let read_guard = self.get_connections().read().await;
        let clients_count = read_guard.len() as u64;
        drop(read_guard);
        clients_count
    }

    #[inline]
    async fn broadcast(&mut self, payload: &Payload) -> Result<(), FastSocketError> {
        let write_guard = self.get_connections().write().await;
        for client in write_guard.values() {
            let socket = client.socket();
            let mut guard = socket.lock().await;
            let result = guard.send(payload).await;
            drop(guard);

            if result.is_err() {
                Log::error(&format!("Failed to send payload: {:?}", result));
            }
        }
        drop(write_guard);
        Ok(())
    }

    #[inline]
    async fn broadcast_to_others(&mut self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError> {
        let socket_id = client.get_socket_id();
        self.broadcast_to_everyone_except(socket_id, payload).await
    }

    #[inline]
    async fn broadcast_to_everyone_except(&mut self, socket_id: &str, payload: &Payload) -> Result<(), FastSocketError> {
        let write_guard = self.get_connections().write().await;
        for (id, client) in write_guard.iter() {
            if id != socket_id {
                let socket = client.socket();
                let mut guard = socket.lock().await;
                let result = guard.send(payload).await;
                drop(guard);

                if result.is_err() {
                    Log::error(&format!("Failed to send payload: {:?}", result));
                }
            }
        }
        Ok(())
    }

    #[inline]
    async fn to_array(&self) -> Value {
        json!({
            "occupied": self.has_connection().await,
            "subscription_count": self.get_clients_count().await,
        })
    }
}
