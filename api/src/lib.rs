mod routers;

use std::net::SocketAddr;

use crate::routers::routers;
use axum::Server;
use yapi_core::config::Config;

#[tokio::main]
async fn start(addr: String) -> anyhow::Result<()> {
    let app = routers();
    let addr: SocketAddr = addr.parse().unwrap();

    log::info!("Server is listening on {}", addr);

    let server = Server::bind(&addr)
        .serve(app.into_make_service());

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub fn main(config: Config) {
    let result = start(config.server_addr);

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}