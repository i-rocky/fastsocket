use std::fmt::Write;
use fastwebsockets::{FragmentCollector, Frame, OpCode, Payload as WsPayload};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use crate::app::App;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::payload::Payload;

pub struct Client<'ud> {
    socket_id: String,
    public_key: String,
    app: &'ud App,
    ws: &'ud mut FragmentCollector<TokioIo<Upgraded>>,
}

impl<'ud> Client<'ud> {
    #[inline]
    pub fn new(ws: &'ud mut FragmentCollector<TokioIo<Upgraded>>, app: &'ud App) -> Self {
        Client {
            app,
            ws,
            socket_id: Self::generate_unique_socket_id(),
            public_key: String::with_capacity(64),
        }
    }

    #[inline(always)]
    pub fn set_public_key(&mut self, public_key: String) {
        self.public_key = public_key;
    }

    #[inline]
    fn generate_unique_socket_id() -> String {
        let mut buffer = String::with_capacity(40);
        write!(&mut buffer, "{:016x}:{:016x}",
               fastrand::u64(..),
               fastrand::u64(..)
        ).unwrap();
        buffer
    }

    #[inline(always)]
    pub fn get_socket_id(&self) -> &str {
        &self.socket_id
    }

    #[inline(always)]
    pub fn get_app(&self) -> &App {
        &self.app
    }

    #[inline]
    pub async fn send(&mut self, payload: &Payload) -> Result<(), FastSocketError> {
        let key = (!self.public_key.is_empty()).then(|| self.public_key.as_str());

        let buffer = payload.compile(key.map(String::from))?;
        let ws_payload = WsPayload::from(buffer);
        let frame = Frame::text(ws_payload);
        self.ws.write_frame(frame)
            .await
            .map_err(|_| FastSocketError::FailedToSendPayloadError)?;

        Ok(())
    }

    #[inline]
    pub async fn pong(&mut self) -> Result<(), FastSocketError> {
        Log::debug("Sending pong");
        let pong_frame = Frame::new(true, OpCode::Pong, None, WsPayload::from(Vec::new()));
        self.ws.write_frame(pong_frame)
            .await
            .map_err(|_| FastSocketError::FailedToSendPongError)?;
        Ok(())
    }
}

impl<'ud> Drop for Client<'ud> {
    fn drop(&mut self) {
        self.public_key.clear();
    }
}

unsafe impl Send for Client<'_> {}
unsafe impl Sync for Client<'_> {}
