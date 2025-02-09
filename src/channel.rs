use crate::client::Client;
use crate::errors::FastSocketError;
use crate::payload::Payload;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait Channel: Send + Sync {
    fn verify_signature(&self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError>;
    async fn save_connection(&mut self, client: Arc<Client>) -> Result<(), FastSocketError>;
    fn has_connection(&self) -> bool;
    fn get_subscribers(&self) -> &HashMap<String, Arc<Client>>;
    async fn subscribe(&mut self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError>;
    async fn unsubscribe(&mut self, socket_id: &str) -> Result<(), FastSocketError>;
    fn get_clients_count(&self) -> u64;
    async fn broadcast(&mut self, payload: &Payload) -> Result<(), FastSocketError>;
    async fn broadcast_to_others(&mut self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError>;
    async fn broadcast_to_everyone_except(&mut self, socket_id: &str, payload: &Payload) -> Result<(), FastSocketError>;
    fn to_array(&self) -> Value;
}
