use crate::channel::Channel;
use crate::channel_manager::ChannelManager;
use crate::client::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct LocalChannelManager {
    channels: HashMap<String, HashMap<String, Arc<RwLock<Box<dyn Channel>>>>>,
}

impl LocalChannelManager {
    #[inline]
    pub fn new() -> Arc<RwLock<Box<dyn ChannelManager>>> {
        Arc::new(RwLock::new(Box::new(Self {
            channels: HashMap::with_capacity(16),
        })))
    }
}

#[async_trait]
impl ChannelManager for LocalChannelManager {
    fn create(&mut self, app_id: &str, channel_name: &str) -> Arc<RwLock<Box<dyn Channel>>> {
        if let Some(channel) = {
            let app_channels = self.channels
                .entry(app_id.to_string())
                .or_insert_with(HashMap::new);
            app_channels.get(channel_name).cloned()
        } {
            return channel;
        }

        let channel: Arc<RwLock<Box<(dyn Channel)>>> = self.make_channel(channel_name);
        self.channels
            .get_mut(app_id)
            .unwrap()
            .insert(channel_name.to_string(), channel.clone());
        channel
    }

    #[inline]
    fn get_channels(&self) -> &HashMap<String, HashMap<String, Arc<RwLock<Box<dyn Channel>>>>> {
        &self.channels
    }

    #[inline]
    async fn remove_from_all_channels(&mut self, client: Arc<Client>) {
        for app_channels in self.channels.values_mut() {
            for channel in app_channels.values() {
                let mut channel = channel.write().await;
                let _ = channel.unsubscribe(client.get_socket_id()).await;
            }
        }
    }
}
