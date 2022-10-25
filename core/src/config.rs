use clap::{Parser, command};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {

    #[clap(long, env)]
    pub server_addr: String,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env, default_value_t = 1)]
    pub database_min_conns: u32,

    #[clap(long, env, default_value_t = 5)]
    pub database_max_conns: u32,
}

pub fn get_config() -> Config {
    Config::parse()
}