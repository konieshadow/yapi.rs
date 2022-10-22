use clap::{Parser, command};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {

    #[clap(long, env)]
    pub server_addr: String,

    #[clap(long, env)]
    pub database_url: String,
}

pub fn get_config() -> Config {
    Config::parse()
}