use fastwebsockets::{FragmentCollector, Frame, OpCode, Payload as WsPayload};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use crate::errors::FastSocketError;
use crate::logger::Log;
use crate::payload::Payload;

pub struct WebsocketConnection {
    ws: FragmentCollector<TokioIo<Upgraded>>,
    public_key: String,
}

impl WebsocketConnection {
    #[inline(always)]
    pub fn new(ws: FragmentCollector<TokioIo<Upgraded>>) -> Self {
        Self {
            ws,
            public_key: String::with_capacity(64),
        }
    }

    #[inline(always)]
    pub async fn write(&mut self, frame: fastwebsockets::Frame<'_>) -> Result<(), fastwebsockets::WebSocketError> {
        self.ws.write_frame(frame).await
    }

    #[inline(always)]
    pub async fn read(&mut self) -> Result<fastwebsockets::Frame<'_>, fastwebsockets::WebSocketError> {
        self.ws.read_frame().await
    }


    #[inline]
    pub async fn send(&mut self, payload: &Payload) -> Result<(), FastSocketError> {
        let key = (!self.public_key.is_empty()).then(|| self.public_key.as_str());

        let buffer = payload.compile(key.map(String::from))?;
        let ws_payload = WsPayload::from(buffer);
        let frame = Frame::text(ws_payload);
        self.write(frame)
            .await
            .map_err(|_| FastSocketError::FailedToSendPayloadError)?;

        Ok(())
    }

    #[inline]
    pub async fn pong(&mut self) -> Result<(), FastSocketError> {
        Log::debug("Sending pong");
        let pong_frame = Frame::new(true, OpCode::Pong, None, WsPayload::from(Vec::new()));
        self.write(pong_frame)
            .await
            .map_err(|_| FastSocketError::FailedToSendPongError)?;
        Ok(())
    }
}
