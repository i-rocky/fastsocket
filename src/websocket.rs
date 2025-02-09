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

#[derive(Clone)]
pub struct WebSocket {
    app_manager: Arc<Box<dyn AppManager>>,
    channel_manager: Arc<Box<dyn ChannelManager>>,
}

impl WebSocket {
    pub fn new(
        app_manager: Arc<Box<dyn AppManager>>,
        channel_manager: Arc<Box<dyn ChannelManager>>,
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
            .add_data("activity_timeout", 5);

        let payload = builder.build();
        if payload.is_err() {
            Log::error("Failed to build payload");
            eprintln!("Failed to build payload {:?}", payload);
            return;
        }

        client
            .get_socket()
            .lock()
            .await
            .send(&payload.unwrap())
            .await
            .expect("Failed to send payload");
    }

    pub async fn on_close(&self, _client: Arc<Client>) {
        Log::debug("Connection closed");
    }

    pub async fn on_message(&self, _client: Arc<Client>) {
        Log::debug("Message received");
    }

    pub async fn on_error(&self, _client: Arc<Client>) {
        Log::debug("Message received");
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
                let result = client.get_socket().lock().await.pong().await;
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
        // let factory = MessageFactory::new(mtx_client.clone(), self.channel_manager.clone()); // TODO: uncomment

        self.on_open(mtx_client.clone()).await;

        loop {
            let lock_mtx = mtx_client.get_socket();
            let mut socket = lock_mtx.lock().await;
            let frame = socket.read().await;
            if frame.is_err() {
                break;
            }

            let payload = self.get_payload(mtx_client.clone(), frame?).await;
            if payload.is_err() {
                Log::error(&format!("Error handling message: {:?}", payload));
                self.on_error(mtx_client.clone()).await;
                break;
            }
        }

        self.on_close(mtx_client.clone()).await;

        Log::debug("Connection closed");

        Ok(())
    }
}
