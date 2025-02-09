use std::sync::Arc;
use fastsocket::json_app_manager::JsonAppManager;
use fastsocket::local_channel_manager::LocalChannelManager;
use fastsocket::websocket::WebSocket;
use fastwebsockets::{upgrade, WebSocketError};
use http_body_util::Empty;
use hyper::body::{Bytes, Incoming};
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio;
use tokio::net::TcpListener;
use fastsocket::app_manager::AppManager;
use fastsocket::errors::FastSocketError;
use fastsocket::logger::Log;

async fn server_upgrade(ws: Arc<Box<WebSocket>>, app_manager: Arc<Box<dyn AppManager>>, mut req: Request<Incoming>) -> Result<Response<Empty<Bytes>>, FastSocketError> {
    let (response, fut) =
        upgrade::upgrade(&mut req).map_err(|_| FastSocketError::UpgradeFailedError)?;

    tokio::task::spawn(async move {
        let path = req.uri().path().to_string();
        let app_id = path.split('/').nth(2);
        if app_id.is_none() {
            Log::error("Invalid path");
            // return Err(FastSocketError::InvalidAppPathError);
            return;
        }
        let app_id = app_id.unwrap();

        let app = app_manager.find(app_id);
        if app.is_none() {
            Log::error(&format!("App not found: {}", app_id));
            // return Err(FastSocketError::InvalidAppError);
            return;
        }
        let app = app.unwrap();
        let handle_future = ws.handle_client(fut, app);
        let pinned_future = Box::pin(handle_future);
        if let Err(e) = tokio::task::unconstrained(pinned_future).await {
            eprintln!("Error handling client: {:?}", e);
        }
    });

    Ok(response)
}

fn main() -> Result<(), WebSocketError> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;

    rt.block_on(async move {
        let listener = TcpListener::bind("127.0.0.1:6002").await?;
        println!("Listening on 127.0.0.1:6002");

        let app_manager = JsonAppManager::new("apps.json").unwrap();
        let channel_manager = LocalChannelManager::new();
        let websocket = WebSocket::new(app_manager.clone(), channel_manager.clone());

        loop {
            let (stream, _) = listener.accept().await?;
            println!("New connection from {}", stream.peer_addr()?);
            let ws = websocket.clone();
            let apm = app_manager.clone();
            tokio::spawn(async move {
                let io = hyper_util::rt::TokioIo::new(stream);
                let conn_fut = hyper::server::conn::http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(|req: Request<Incoming>| {
                            let wsc = ws.clone();
                            let apmc = apm.clone();
                            async move { server_upgrade(wsc, apmc, req).await }
                        }),
                    )
                    .with_upgrades();
                if let Err(e) = conn_fut.await {
                    println!("Connection error: {:?}", e);
                }
            });
        }
    })
}
