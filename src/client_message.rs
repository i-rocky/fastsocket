use std::future::Future;
use std::pin::Pin;
use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::message::Message;
use crate::payload::Payload;
use std::sync::Arc;
use futures::FutureExt;

pub struct ClientMessage {
    client: Arc<Client>,
    payload: Payload,
    channel_manager: Arc<Box<dyn ChannelManager>>,
}

impl ClientMessage {
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

impl Message for ClientMessage {
    fn respond<'a>(&'a mut self) -> Pin<Box<dyn Future<Output=Result<(), FastSocketError>> + Send + 'a>> {
        async move {
            Log::debug("Received client message");
            // Implementation here
            Ok(())
        }.boxed()
    }
}
