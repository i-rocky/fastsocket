use crate::channel::Channel;
use crate::client::Client;
use crate::private_channel::PrivateChannel;
use crate::public_channel::PublicChannel;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::encrypted_channel::EncryptedChannel;
use crate::presence_channel::PresenceChannel;

#[async_trait]
pub trait ChannelManager: Send + Sync {
    fn make_channel(&self, channel_name: &str) -> Arc<RwLock<Box<dyn Channel>>> {
        if channel_name.starts_with("private-encrypted-") {
            return Arc::new(RwLock::new(Box::new(EncryptedChannel::new(
                channel_name.to_string(),
            ))));
        }

        if channel_name.starts_with("private-") {
            return Arc::new(RwLock::new(Box::new(PrivateChannel::new(
                channel_name.to_string(),
            ))));
        }

        if channel_name.starts_with("presence-") {
            return Arc::new(RwLock::new(Box::new(PresenceChannel::new(
                channel_name.to_string(),
            ))));
        }

        Arc::new(RwLock::new(Box::new(PublicChannel::new(
            channel_name.to_string(),
        ))))
    }
    fn create(&mut self, app_id: &str, channel_name: &str) -> Arc<RwLock<Box<dyn Channel>>>;
    fn find_or_create(
        &mut self,
        app_id: &str,
        channel_name: &str,
    ) -> Arc<RwLock<Box<dyn Channel>>> {
        self.find(app_id, channel_name)
            .unwrap_or_else(|| self.create(app_id, channel_name))
    }
    fn find(&self, app_id: &str, channel_name: &str) -> Option<Arc<RwLock<Box<dyn Channel>>>> {
        self.get_channels()
            .get(app_id)
            .and_then(|channels| channels.get(channel_name))
            .cloned()
    }
    fn get_channels(&self) -> &HashMap<String, HashMap<String, Arc<RwLock<Box<dyn Channel>>>>>;
    async fn remove_from_all_channels(&mut self, client: Arc<Client>);
}
