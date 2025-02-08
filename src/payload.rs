use serde_json::{Map, Value};
use crate::errors::FastSocketError;

#[derive(Debug)]
pub struct Payload {
    event: String,
    channel: String,
    data: Map<String, Value>,
}

impl Payload {
    #[inline]
    pub fn new(json_data: &str) -> Result<Self, FastSocketError> {
        let payload: Value = serde_json::from_str(&json_data)
            .map_err(|_| FastSocketError::InvalidMessageError)?;

        let obj = payload.as_object()
            .ok_or(FastSocketError::InvalidMessageError)?;

        let event = obj.get("event")
            .and_then(Value::as_str)
            .ok_or(FastSocketError::InvalidMessageError)?
            .to_string();

        let channel = obj.get("channel")
            .and_then(Value::as_str)
            .ok_or(FastSocketError::InvalidMessageError)?
            .to_string();

        let data = obj.get("data")
            .and_then(Value::as_object)
            .map(|m| m.clone())
            .unwrap_or_else(Map::new);

        Ok(Payload {
            event,
            channel,
            data,
        })
    }

    #[inline(always)]
    pub fn get_event(&self) -> &str {
        &self.event
    }

    #[inline(always)]
    pub fn get_channel(&self) -> &str {
        &self.channel
    }

    #[inline(always)]
    pub fn get_data(&self) -> &Map<String, Value> {
        &self.data
    }

    #[inline]
    pub fn get_data_str(&self, key: &str) -> Option<String> {
        self.data.get(key)
            .and_then(Value::as_str)
            .map(|s| s.to_string())
    }

    #[inline]
    pub fn get_data_int(&self, key: &str) -> Option<i64> {
        self.data.get(key)
            .and_then(Value::as_i64)
    }

    #[inline]
    pub fn get_data_bool(&self, key: &str) -> Option<bool> {
        self.data.get(key)
            .and_then(Value::as_bool)
    }

    #[inline]
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn builder() -> PayloadBuilder {
        PayloadBuilder::default()
    }
}

#[derive(Default, Debug)]
pub struct PayloadBuilder {
    event: Option<String>,
    channel: Option<String>,
    data: Map<String, Value>,
}

impl PayloadBuilder {
    #[inline]
    pub fn event<S: Into<String>>(mut self, event: S) -> Self {
        self.event = Some(event.into());
        self
    }

    #[inline]
    pub fn channel<S: Into<String>>(mut self, channel: S) -> Self {
        self.channel = Some(channel.into());
        self
    }

    #[inline]
    pub fn data(mut self, data: Map<String, Value>) -> Self {
        self.data = data;
        self
    }

    #[inline]
    pub fn add_data<S: Into<String>, V: Into<Value>>(mut self, key: S, value: V) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<Payload, FastSocketError> {
        let event = self.event
            .ok_or(FastSocketError::InvalidMessageError)?;
        let channel = self.channel
            .ok_or(FastSocketError::InvalidMessageError)?;

        Ok(Payload {
            event,
            channel,
            data: self.data,
        })
    }
}
