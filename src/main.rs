use poem::{listener::TcpListener, Server};

use config::database::*;
use controller::user_controller::config_endpoints;
use dotenv::dotenv;
use log::info;
use std::env;

mod config;
mod controller;
mod model;
mod repository;
mod schema;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let log_config_file =
        env::var("LOG4RS_CONFIG_FILE").unwrap_or("logging_config.yaml".to_string());
    log4rs::init_file(log_config_file, Default::default()).unwrap();

    unsafe {
        info!("DB pool - starting...");
        DB_POOL = Some(create_db_pool().await);
        info!("DB pool - started OK: {:?}", DB_POOL.as_ref().unwrap());
    }

    info!("Poem server - starting...");
    Server::new(TcpListener::bind(config::constants::HOST))
        .run(config_endpoints())
        .await
        .expect("Poem server fail");
    info!("Poem server - started OK");

    info!("app started OK");
}
