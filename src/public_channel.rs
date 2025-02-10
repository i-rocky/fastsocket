use crate::channel::Channel;
use crate::client::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PublicChannel {
    name: String,
    connections: RwLock<HashMap<String, Arc<Client>>>,
}

impl PublicChannel {
    #[inline]
    pub fn new(name: String) -> Self {
        Self {
            name,
            connections: RwLock::new(HashMap::with_capacity(32)),
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl Channel for PublicChannel {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_connections(&self) -> &RwLock<HashMap<String, Arc<Client>>> {
        &self.connections
    }
}
