use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Request, Response, Body, Server};
use hyper::service::{service_fn, make_service_fn};

async fn handle_request(_req : Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, world!".into()))
}

#[tokio::main]
async fn main() {
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 12181));

    let make_callback = make_service_fn(|_connection| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&socket_addr).serve(make_callback);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
