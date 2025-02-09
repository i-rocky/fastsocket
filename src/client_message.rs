use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::message::Message;
use crate::payload::Payload;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ClientMessage {
    client: Arc<Client>,
    payload: Payload,
    channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
}

impl ClientMessage {
    pub fn new(
        client: Arc<Client>,
        payload: Payload,
        channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
    ) -> Self {
        Self {
            client,
            payload,
            channel_manager,
        }
    }
}

#[async_trait]
impl Message for ClientMessage {
    async fn respond(&self) -> Result<(), FastSocketError> {
        Log::debug("Received client message");
        // Implementation here
        Ok(())
    }
}
