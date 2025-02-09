use crate::errors::FastSocketError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
pub struct App {
    id: String,
    key: String,
    secret: String,
    name: String,
    host: String,
    path: String,
    capacity: u64,
    connection_count: u64,
    flags: u8,
}

impl App {
    const CLIENT_MESSAGES_FLAG: u8 = 1 << 0;
    const STATISTICS_FLAG: u8 = 1 << 1;

    #[inline]
    pub fn new(
        id: String,
        key: String,
        secret: String,
        name: String,
        host: String,
        path: String,
        capacity: u64,
        flags: u8,
    ) -> Result<Arc<Self>, FastSocketError> {
        if id.is_empty() {
            return Err(FastSocketError::InvalidAppIdError);
        }

        if key.is_empty() {
            return Err(FastSocketError::InvalidAppKeyError);
        }

        if secret.is_empty() {
            return Err(FastSocketError::InvalidAppSecretError);
        }

        if name.is_empty() {
            return Err(FastSocketError::InvalidAppNameError);
        }

        if host.is_empty() {
            return Err(FastSocketError::InvalidAppHostError);
        }

        if path.is_empty() {
            return Err(FastSocketError::InvalidAppPathError);
        }

        if capacity == 0 {
            return Err(FastSocketError::InvalidAppCapacityError);
        }

        Ok(Arc::new(App {
            id,
            key,
            secret,
            name,
            host,
            path,
            capacity,
            flags,
            connection_count: 0,
        }))
    }

    #[inline]
    pub fn arc(self) -> Arc<App> {
        Arc::new(self)
    }

    #[inline]
    pub fn to_app(&self) -> Self {
        Self {
            id: self.id.clone(),
            key: self.key.clone(),
            secret: self.secret.clone(),
            name: self.name.clone(),
            host: self.host.clone(),
            path: self.path.clone(),
            capacity: self.capacity,
            flags: self.flags,
            connection_count: self.connection_count,
        }
    }

    #[inline]
    pub fn get_id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn get_key(&self) -> &str {
        &self.key
    }

    #[inline]
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn get_host(&self) -> &str {
        &self.host
    }

    #[inline]
    pub fn get_path(&self) -> &str {
        &self.path
    }

    #[inline]
    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }

    #[inline]
    pub fn set_capacity(&mut self, capacity: u64) {
        self.capacity = capacity;
    }

    #[inline]
    pub fn get_connection_count(&self) -> u64 {
        self.connection_count
    }

    #[inline]
    pub fn increment_connection_count(&mut self) {
        self.connection_count += 1;
    }

    #[inline]
    pub fn decrement_connection_count(&mut self) {
        self.connection_count -= 1;
    }

    #[inline]
    pub fn enable_client_messages(&mut self, enabled: bool) {
        if enabled {
            self.flags |= Self::CLIENT_MESSAGES_FLAG;
        } else {
            self.flags &= !Self::CLIENT_MESSAGES_FLAG;
        }
    }

    #[inline]
    pub fn is_client_messages_enabled(&self) -> bool {
        self.flags & Self::CLIENT_MESSAGES_FLAG != 0
    }

    #[inline]
    pub fn enable_statistics(&mut self, enabled: bool) {
        if enabled {
            self.flags |= Self::STATISTICS_FLAG;
        } else {
            self.flags &= !Self::STATISTICS_FLAG;
        }
    }

    #[inline]
    pub fn is_statistics_enabled(&self) -> bool {
        self.flags & Self::STATISTICS_FLAG != 0
    }
}
