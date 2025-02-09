use crate::channel_manager::ChannelManager;
use crate::channel_protocol_message::ChannelProtocolMessage;
use crate::client::Client;
use crate::client_message::ClientMessage;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::message::Message;
use crate::payload::Payload;
use std::sync::Arc;

pub struct MessageFactory {
    client: Arc<Client>,
    channel_manager: Arc<Box<dyn ChannelManager>>,
}

impl MessageFactory {
    pub fn new(
        client: Arc<Client>,
        channel_manager: Arc<Box<dyn ChannelManager>>,
    ) -> Arc<Box<Self>> {
        Arc::new(Box::new(Self {
            client,
            channel_manager,
        }))
    }

    pub fn for_payload(&self, payload: Payload) -> Result<Arc<Box<dyn Message>>, FastSocketError> {
        if payload.get_event().starts_with("pusher:") {
            Log::debug("Received pusher message");
            Ok(Arc::new(Box::new(ChannelProtocolMessage::new(
                self.client.clone(),
                payload,
                self.channel_manager.clone(),
            ))))
        } else {
            Log::debug("Received client message");
            Ok(Arc::new(Box::new(ClientMessage::new(
                self.client.clone(),
                payload,
                self.channel_manager.clone(),
            ))))
        }
    }
}
