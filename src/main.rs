use fastwebsockets::{upgrade, Frame, OpCode, WebSocketError};
use http_body_util::Empty;
use hyper::body::{Bytes, Incoming};
use hyper::service::service_fn;
use hyper::Response;
use tokio;
use tokio::net::TcpListener;
use std::sync::atomic::{AtomicU64, Ordering};
use fastsocket::logger::Log;

static ACTIVE_CONNECTIONS: AtomicU64 = AtomicU64::new(0);
const MAX_CONNECTIONS: u64 = 10000;

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
    
    loop {
        let frame = match ws.read_frame().await {
            Ok(frame) => frame,
            Err(e) => {
                Log::error(&format!("Error reading frame: {:?}", e));
                break;
            }
        };

        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
                if let Err(e) = ws.write_frame(frame).await {
                    Log::error(&format!("Error writing frame: {:?}", e));
                    break;
                }
            }
            OpCode::Ping => {
                if let Err(e) = ws.write_frame(Frame::pong(frame.payload)).await {
                    Log::error(&format!("Error sending pong: {:?}", e));
                    break;
                }
            }
            _ => {}
        }
    }

    Log::debug("Connection closed");
    Ok(())
}

async fn server_upgrade(
    mut req: hyper::Request<Incoming>,
) -> Result<Response<Empty<Bytes>>, WebSocketError> {
    let current = ACTIVE_CONNECTIONS.load(Ordering::Relaxed);
    if current >= MAX_CONNECTIONS {
        return Ok(Response::builder()
            .status(503)
            .body(Empty::new())
            .unwrap());
    }

    let (response, fut) = upgrade::upgrade(&mut req)?;
    
    ACTIVE_CONNECTIONS.fetch_add(1, Ordering::Relaxed);
    
    tokio::task::spawn(async move {
        if let Err(e) = tokio::task::unconstrained(handle_client(fut)).await {
            eprintln!("Error handling client: {:?}", e);
        }
        ACTIVE_CONNECTIONS.fetch_sub(1, Ordering::Relaxed);
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
        loop {
            let (stream, _) = listener.accept().await?;
            println!("New connection from {}", stream.peer_addr()?);
            tokio::spawn(async move {
                let io = hyper_util::rt::TokioIo::new(stream);
                let conn_fut = hyper::server::conn::http1::Builder::new()
                    .serve_connection(io, service_fn(server_upgrade))
                    .with_upgrades();
                if let Err(e) = conn_fut.await {
                    println!("Connection error: {:?}", e);
                }
            });
        }
    })
}
