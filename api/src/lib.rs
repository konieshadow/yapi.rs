mod routers;

use std::{net::SocketAddr, sync::Arc};

use crate::routers::routers;
use axum::{Server, Extension};
use sea_orm::DatabaseConnection;
use yapi_core::{config::Config, Context};

pub async fn start(config: Config, db: DatabaseConnection) -> anyhow::Result<()> {
    let context = Context {
        config: Arc::new(config.clone()),
        db,
    };

    let app = routers()
        .layer(Extension(context));

    let addr: SocketAddr = config.server_addr.parse().unwrap();

    let server = Server::bind(&addr)
        .serve(app.into_make_service());

    let graceful = server.with_graceful_shutdown(shutdown_signal());
    
    log::info!("server is listening on {}", addr);

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