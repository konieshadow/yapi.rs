use std::net::SocketAddr;

use axum::{Router, routing::get};
use yapi_core::config::Config;

#[tokio::main]
async fn start(addr: &str) -> anyhow::Result<()> {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr: SocketAddr = String::from(addr).parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub fn main(config: Config) {
    let result = start(&config.server_addr);

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}