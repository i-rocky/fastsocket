use crate::channel::Channel;
use crate::client::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{Map, Value};
use tokio::sync::RwLock;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::payload::Payload;

pub struct PresenceChannel {
    name: String,
    connections: RwLock<HashMap<String, Arc<Client>>>,
    channel_data: RwLock<HashMap<String, Value>>,
}

impl PresenceChannel {
    #[inline]
    pub fn new(name: String) -> Self {
        Self {
            name,
            connections: RwLock::new(HashMap::with_capacity(32)),
            channel_data: RwLock::new(HashMap::with_capacity(32)),
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    async fn channel_data(&self) -> Map<String, Value> {
        let mut map = Map::new();
        let mut presence = Map::new();
        presence.insert(String::from("ids"), self.get_client_ids().await); // all user_id from channel_data
        presence.insert(String::from("hash"), self.get_hash().await); // all channel_data
        presence.insert(String::from("count"), Value::from(self.get_clients_count().await));
        map.insert(String::from("presence"), Value::Object(presence));
        map
    }

    #[inline]
    async fn get_client_ids(&self) -> Value {
        let mut ids = Vec::new();
        for (_, val) in self.channel_data.read().await.iter() {
            ids.push(val.get("user_id").unwrap().clone());
        }
        Value::Array(ids)
    }

    #[inline]
    async fn get_hash(&self) -> Value {
        let mut hash = Map::new();
        for (_, val) in self.channel_data.read().await.iter() {
            hash.insert(String::from(val.get("user_id").unwrap().to_string()), val.get("user_info").unwrap().clone());
        }
        Value::Object(hash)
    }
}

#[async_trait]
impl Channel for PresenceChannel {
    #[inline]
    fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn get_connections(&self) -> &RwLock<HashMap<String, Arc<Client>>> {
        &self.connections
    }

    #[inline]
    async fn subscribe(&mut self, client: Arc<Client>, payload: &Payload) -> Result<(), FastSocketError> {
        let result = self.verify_signature(client.clone(), payload).await;
        if result.is_err() {
            return Err(FastSocketError::InvalidSignatureError)
        }

        Log::debug("Subscribing");
        self.save_connection(client.clone()).await?;

        let channel_data: Value = serde_json::from_str(payload.get_data_str("channel_data").unwrap()).unwrap();

        self.channel_data.get_mut().insert(client.get_socket_id().to_string(), channel_data.clone());

        let response = Payload::builder()
            .event("pusher_internal:subscription_succeeded")
            .channel(self.get_name())
            .data(self.channel_data().await)
            .build();
        if response.is_err() {
            return Err(response.err().unwrap());
        }

        let socket = client.socket();
        let mut guard = socket.lock().await;
        guard.send(&response.unwrap()).await?;
        drop(guard);

        let kv_channel_data = channel_data.as_object().unwrap();
        let event = Payload::builder()
            .event("pusher_internal:member_added")
            .channel(self.get_name())
            .data(kv_channel_data.clone())
            .build();

        self.broadcast_to_others(client, &event?).await
    }

    #[inline]
    async fn unsubscribe(&mut self, socket_id: &str) -> Result<(), FastSocketError> {
        self.channel_data.get_mut().remove(socket_id);
        self.default_unsubscribe(socket_id).await
    }
}
