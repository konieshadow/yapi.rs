use error::Error;

pub mod config;
pub mod error;
pub mod res;
pub mod services;
pub mod extractors;

pub type Result<T, E = Error> = std::result::Result<T, E>;