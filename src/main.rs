use yapi_core::config;

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let config = config::get_config();
    yapi_api::main(config)
}