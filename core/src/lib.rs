use std::sync::Arc;

use config::Config;
use error::Error;
use sea_orm::DatabaseConnection;

pub mod config;
pub mod error;
pub mod res;
pub mod services;
pub mod extractors;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub db: DatabaseConnection,
}