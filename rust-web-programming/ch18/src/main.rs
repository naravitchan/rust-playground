use bytes::{BufMut, BytesMut};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingBody {
    pub one: String,
    pub two: i32,
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, &'static str> {
    let bytes = body::to_bytes(req.into_body()).await.unwrap();
    let response_body: IncomingBody = serde_json::from_slice(&bytes).unwrap();
    let mut buf = BytesMut::new().writer();
    serde_json::to_writer(&mut buf, &response_body).unwrap();
    Ok(Response::new(Body::from(buf.into_inner().freeze()))) // freeze?
}

#[tokio::main]
async fn main() {
    let app_type = env::var("APP_TYPE").unwrap();
    match app_type.as_str() {
        "server" => {
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            let server = Server::bind(&addr).serve(make_service_fn(|_conn| async {
                Ok::<_, hyper::Error>(service_fn(move |req| async { handle(req).await }))
            }));
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        }
        "worker" => {
            println!("worker not defined yet");
        }
        _ => {
            panic!("{} app type not supported", app_type);
        }
    }
}
