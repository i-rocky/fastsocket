use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::message::Message;
use crate::payload::Payload;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ChannelProtocolMessage {
    client: Arc<Client>,
    payload: Payload,
    channel_manager: Arc<Box<dyn ChannelManager>>,
}

impl ChannelProtocolMessage {
    pub fn new(
        client: Arc<Client>,
        payload: Payload,
        channel_manager: Arc<Box<dyn ChannelManager>>,
    ) -> Self {
        Self {
            client,
            payload,
            channel_manager,
        }
    }
}

#[async_trait]
impl Message for ChannelProtocolMessage {
    async fn respond(&self) -> Result<(), FastSocketError> {
        Log::debug(&format!("Received channel protocol message: {:?}", self.payload));
        match self.payload.get_event() {
            "pusher:ping" => {
                Log::debug("Received ping");
                let result = self.client.get_socket().lock().await.pong().await;
                if result.is_err() {
                    Log::error(&format!("Error sending pong: {:?}", result));
                }
                Log::debug("Pong sent");
                Ok(())
            }
            &_ => Ok(()),
        }
    }
}
