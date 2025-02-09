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

impl Message for ChannelProtocolMessage {
    fn respond<'a>(&'a mut self) -> Pin<Box<dyn Future<Output=Result<(), FastSocketError>> + Send + 'a>> {
        async move {
            match self.payload.get_event() {
                "pusher:ping" => {
                    let result = self.client
                        .get_socket()
                        .lock()
                        .await
                        .pong()
                        .await;
                    if result.is_err() {
                        Log::error(&format!("Error sending pong: {:?}", result));
                    }

                    Ok(())
                }
                &_ => Ok(()),
            }
        }.boxed()
    }
}
