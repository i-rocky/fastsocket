use crate::app::App;
use crate::app_manager::AppManager;
use crate::channel_manager::ChannelManager;
use crate::client::Client;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::payload::{Payload, PayloadBuilder};
use crate::websocket_connection::WebsocketConnection;
use fastwebsockets::{upgrade, OpCode, WebSocketError};
use std::io::Read;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::message_factory::MessageFactory;

#[derive(Clone)]
pub struct WebSocket {
    app_manager: Arc<Box<dyn AppManager>>,
    channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
}

impl WebSocket {
    pub fn new(
        app_manager: Arc<Box<dyn AppManager>>,
        channel_manager: Arc<RwLock<Box<dyn ChannelManager>>>,
    ) -> Arc<Box<Self>> {
        Arc::new(Box::new(Self {
            app_manager,
            channel_manager,
        }))
    }

    pub async fn on_open(&self, client: Arc<Client>) {
        Log::debug("Connection opened");

        let builder = PayloadBuilder::default()
            .event("pusher:connection_established")
            .add_data("socket_id", client.get_socket_id())
            .add_data("activity_timeout", 30);

        let payload = builder.build();
        if payload.is_err() {
            Log::error("Failed to build payload");
            eprintln!("Failed to build payload {:?}", payload);
            return;
        }
        let socket = client.get_socket();
        let mut guard = socket
            .lock()
            .await;

        let result = guard
            .send(&payload.unwrap())
            .await;

        drop(guard);

        if result.is_err() {
            Log::error(&format!("Failed to send payload: {:?}", result));
            return;
        }
    }

    pub async fn on_close(&self, _client: Arc<Client>) {
        Log::debug("Connection closed");
    }

    pub async fn on_error(&self, _client: Arc<Client>) {
        Log::debug("Error occurred");
    }

    pub async fn get_payload(
        &self,
        client: Arc<Client>,
        frame: fastwebsockets::Frame<'_>,
    ) -> Result<Option<Payload>, FastSocketError> {
        match frame.opcode {
            OpCode::Close => Err(FastSocketError::ConnectionClosed),
            OpCode::Text | OpCode::Binary => {
                assert!(frame.fin);
                let c: Result<Vec<u8>, _> = frame.payload.bytes().collect();
                if let Err(e) = c {
                    Log::error(&format!("Error reading payload: {:?}", e));
                    self.on_error(client).await;
                    return Err(FastSocketError::ErrorReadingPayload);
                }
                let content = String::from_utf8(c.unwrap());

                if content.is_err() {
                    Log::error(&format!("Error transforming payload: {:?}", content));
                    self.on_error(client).await;
                    return Err(FastSocketError::ErrorDecodingPayload);
                }
                let content = content.unwrap();
                Log::debug(&format!("Received message: {:?}", content));
                Ok(Option::from(Payload::new(content.as_str())?))
            }
            OpCode::Ping => {
                let socket = client.get_socket();
                let mut guard = socket.lock().await;
                let result = guard.pong().await;
                drop(guard);

                if result.is_err() {
                    Log::error(&format!("Error sending pong: {:?}", result));
                    self.on_error(client).await;
                    return Err(FastSocketError::ErrorSendingPong);
                }

                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub async fn handle_client(
        &self,
        fut: upgrade::UpgradeFut,
        app: Arc<App>,
    ) -> Result<(), WebSocketError> {
        let ws = fastwebsockets::FragmentCollector::new(fut.await?);
        let client = Client::new(
            WebsocketConnection::new(ws),
            app,
            self.channel_manager.clone(),
        );
        let mtx_client = Arc::new(client);
        let factory = MessageFactory::new(mtx_client.clone(), self.channel_manager.clone());

        self.on_open(mtx_client.clone()).await;

        loop {
            let lock_mtx = mtx_client.get_socket();
            let mut guard = lock_mtx.lock().await;
            let frame = guard.read().await;

            if frame.is_err() {
                drop(guard);
                break;
            }

            let payload = self.get_payload(mtx_client.clone(), frame?).await;
            drop(guard);
            if payload.is_err() {
                Log::error(&format!("Error getting payload: {:?}", payload));
                self.on_error(mtx_client.clone()).await;
                continue;
            }

            let payload = payload.unwrap();

            if payload.is_none() {
                continue;
            }

            let msg = factory.for_payload(payload.unwrap());
            if msg.is_err() {
                Log::error(&format!("Error creating message: {:?}", msg.err().unwrap()));
                self.on_error(mtx_client.clone()).await;
                continue;
            }

            let responder = msg.unwrap();
            let result = responder.respond().await;
            if result.is_err() {
                Log::error(&format!("Error responding: {:?}", result));
                self.on_error(mtx_client.clone()).await;
                continue;
            }
        }

        self.on_close(mtx_client.clone()).await;

        Log::debug("Connection closed");

        Ok(())
    }
}
