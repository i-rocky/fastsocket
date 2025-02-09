use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::message::Message;
use crate::payload::Payload;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::channel::Channel;

pub struct ChannelProtocolMessage {
    client: Arc<Client>,
    payload: Payload,
    channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
}

impl ChannelProtocolMessage {
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
impl Message for ChannelProtocolMessage {
    async fn respond(&self) -> Result<(), FastSocketError> {
        Log::debug(&format!("Received channel protocol message: {:?}", self.payload));
        match self.payload.get_event() {
            "pusher:ping" => {
                Log::debug("Received ping");
                let socket = self.client.get_socket();
                let mut guard = socket.lock().await;
                let result = guard.pong().await;
                drop(guard);

                if result.is_err() {
                    Log::error(&format!("Error sending pong: {:?}", result));
                }
                Log::debug("Pong sent");
                Ok(())
            }
            "pusher:subscribe" => {
                Log::debug("Received subscribe");
                let channel_name = self.payload.get_data_str("channel");
                if channel_name.is_none() {
                    Log::error("Invalid channel name");
                    return Ok(());
                }
                let channel_name = channel_name.unwrap();
                let mut read_guard = self.channel_manager.read().await;
                let e_channel = read_guard.find(self.client.get_app().get_id(), channel_name);
                drop(read_guard);
                let channel = if e_channel.is_some() {
                    Log::debug(&format!("Found channel: {}", channel_name));
                    e_channel.unwrap()
                } else {
                let mut write_guard = self.channel_manager.write().await;
                    let channel = write_guard.find_or_create(self.client.get_app().get_id(), channel_name);
                    drop(write_guard);
                    Log::debug(&format!("Created channel: {}", channel_name));
                    channel
                };

                Log::debug(&format!("Subscribing to channel: {}", channel_name));
                channel.write().await.subscribe(self.client.clone(), &self.payload).await?;
                Log::debug(&format!("Subscribed to channel: {}", channel_name));

                Ok(())
            }
            "pusher:unsubscribe" => {
                Log::debug("Received unsubscribe");
                let channel_name = self.payload.get_channel();
                let mut read_guard = self.channel_manager.read().await;
                let e_channel = read_guard.find(self.client.get_app().get_id(), channel_name);
                drop(read_guard);
                if e_channel.is_none() {
                    Log::debug(&format!("Channel not found: {}", channel_name));
                    return Ok(());
                }
                let channel = e_channel.unwrap();

                Log::debug(&format!("Unsubscribing from channel: {}", channel_name));
                channel.write().await.unsubscribe(self.client.get_socket_id()).await?;
                Log::debug(&format!("Unsubscribed from channel: {}", channel_name));

                Ok(())
            }
            &_ => Ok(()),
        }
    }
}
