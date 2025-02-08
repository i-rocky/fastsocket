use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::payload::Payload;

#[async_trait]
pub trait Channel: Send + Sync {
    fn verify_signature(&self, client: &Client<'_>, payload: &Payload) -> Result<(), FastSocketError>;
    async fn save_connection(&mut self, client: Client<'_>) -> Result<(), FastSocketError>;
    fn has_connection(&self) -> bool;
    fn get_subscribers(&self) -> &HashMap<String, Arc<RwLock<Client<'static>>>>;
    async fn subscribe(&mut self, client: &Client<'_>, payload: &Payload) -> Result<(), FastSocketError>;
    async fn unsubscribe(&mut self, socket_id: &str) -> Result<(), FastSocketError>;
    fn get_clients_count(&self) -> u64;
    async fn broadcast(&mut self, payload: &Payload) -> Result<(), FastSocketError>;
    async fn broadcast_to_others(&mut self, client: &Client<'_>, payload: &Payload) -> Result<(), FastSocketError>;
    async fn broadcast_to_everyone_except(&mut self, socket_id: &str, payload: &Payload) -> Result<(), FastSocketError>;
    fn to_array(&self) -> Value;
}