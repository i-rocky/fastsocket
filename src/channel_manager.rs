use crate::channel::Channel;
use crate::client::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait ChannelManager: Send + Sync {
    fn find_or_create(&mut self, app_id: &str, channel_name: &str) -> Arc<RwLock<Box<dyn Channel>>>;
    fn find(&self, app_id: &str, channel_name: &str) -> Option<Arc<RwLock<Box<dyn Channel>>>>;
    fn get_channels(&self) -> &HashMap<String, HashMap<String, Arc<RwLock<Box<dyn Channel>>>>>;
    async fn remove_from_all_channels(&mut self, client: Arc<Client>);
}
