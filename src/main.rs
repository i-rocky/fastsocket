use fastwebsockets::{upgrade, OpCode, WebSocketError};
use http_body_util::Empty;
use hyper::body::{Bytes, Incoming};
use hyper::service::service_fn;
use hyper::Response;
use tokio;
use tokio::net::TcpListener;

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);

    loop {
        let frame = ws.read_frame().await?;
        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
                ws.write_frame(frame).await?;
            }
            _ => {}
        }
    }

    println!("connection closed");

    Ok(())
}

async fn server_upgrade(
    mut req: hyper::Request<Incoming>,
) -> Result<Response<Empty<Bytes>>, WebSocketError> {
    let (response, fut) = upgrade::upgrade(&mut req)?;

    tokio::task::spawn(async move {
        if let Err(e) = tokio::task::unconstrained(handle_client(fut)).await {
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
