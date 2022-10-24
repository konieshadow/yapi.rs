use error::Error;

pub mod config;
pub mod error;
pub mod res;
pub mod services;
pub mod types;
pub mod extractors;

#[macro_use]
extern crate lazy_static;

pub type Result<T, E = Error> = std::result::Result<T, E>;