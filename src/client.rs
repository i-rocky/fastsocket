use crate::app::App;
use crate::channel_manager::ChannelManager;
use crate::websocket_connection::WebsocketConnection;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct Client {
    socket_id: String,
    public_key: String,
    app: Arc<App>,
    ws: Arc<Mutex<WebsocketConnection>>,
    channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
}

impl Client {
    #[inline]
    pub fn new(
        ws: WebsocketConnection,
        app: Arc<App>,
        channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
    ) -> Self {
        Self {
            app,
            ws: Arc::new(Mutex::new(ws)),
            socket_id: Self::generate_unique_socket_id(),
            public_key: String::with_capacity(64),
            channel_manager,
        }
    }

    pub fn socket(&self) -> Arc<Mutex<WebsocketConnection>> {
        self.ws.clone()
    }

    #[inline(always)]
    pub fn set_public_key(&mut self, public_key: String) {
        self.public_key = public_key;
    }

    #[inline]
    fn generate_unique_socket_id() -> String {
        let a = fastrand::u64(0..1_000_000);
        let b = fastrand::u64(0..1_000_000);
        format!("{:06}.{:06}", a, b)
    }

    #[inline(always)]
    pub fn get_socket_id(&self) -> &str {
        &self.socket_id
    }

    #[inline(always)]
    pub fn get_app(&self) -> Arc<App> {
        self.app.clone()
    }

    #[inline(always)]
    pub fn get_socket(&self) -> Arc<Mutex<WebsocketConnection>> {
        self.ws.clone()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.public_key.clear();
    }
}
