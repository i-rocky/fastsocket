use crate::channel::Channel;
use crate::client::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::errors::FastSocketError;
use crate::payload::Payload;

pub struct EncryptedChannel {
    name: String,
    connections: RwLock<HashMap<String, Arc<Client>>>,
}

impl EncryptedChannel {
    #[inline]
    pub fn new(name: String) -> Self {
        Self {
            name,
            connections: RwLock::new(HashMap::with_capacity(32)),
        }
    }
}

#[async_trait]
impl Channel for EncryptedChannel {
    #[inline]
    fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn get_connections(&self) -> &RwLock<HashMap<String, Arc<Client>>> {
        &self.connections
    }

    #[inline]
    async fn subscribe(&mut self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError> {
        let result = self.verify_signature(client.clone(), payload).await;
        if result.is_err() {
            return Err(FastSocketError::InvalidSignatureError)
        }

        self.default_subscribe(client, payload).await
    }
}
