use std::collections::HashMap;
use crate::channel::Channel;
use crate::client::Client;

pub trait ChannelManager {
    fn find_or_create(&self, app_id: &str, channel_name: &str) -> &dyn Channel;
    fn find(&self, app_id: &str, channel_name: &str) -> Option<&dyn Channel>;
    fn get_channels(&self) -> HashMap<String, Vec<&dyn Channel>>;
    fn remove_from_all_channels(&self, client: &Client);
}
