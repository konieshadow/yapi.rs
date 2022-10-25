use sea_orm::{Database, ConnectOptions};
use yapi_core::config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let config = config::get_config();

    // connection pool
    let mut conn_opt = ConnectOptions::new(config.database_url.to_owned());
    conn_opt.min_connections(config.database_min_conns);
    conn_opt.max_connections(config.database_max_conns);
    let db = Database::connect(conn_opt).await.expect("connect to database failed");

    yapi_api::start(config, db).await
}