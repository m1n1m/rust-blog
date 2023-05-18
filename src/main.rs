
mod models;
mod schema;
mod logs;
mod database;
mod modules;

use std::sync::{Mutex, RwLock};
use actix_web::HttpServer;
use actix_web::App;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::ServiceConfig;
use slog::{info};
use crate::database::{Database, DBPool};
use crate::logs::LOGGER;
use crate::modules::ModuleRegistry;

// static DB_POOL_SINGLETONE: RwLock<Vec<DBPool>> = RwLock::new(Vec::new());
//
// fn set_db_pool(pool: DBPool) {
//     DB_POOL_SINGLETONE.write().unwrap().push(pool.clone());
// }

// fn get_db_pool() -> &'static DBPool {
//     DB_POOL_SINGLETONE.read().unwrap().get(0).unwrap()
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database = Database::new();
    let pool: DBPool = database.get_pool();
    // set_db_pool(pool);

    let server_url = std::env::var("SERVER_URL").unwrap();

    info!(LOGGER, "Starting the server at {}", server_url);

    static MODULE_REGISTRY: ModuleRegistry = ModuleRegistry {};

    let configure_actix_main = |actix_cfg: &mut ServiceConfig| {
        MODULE_REGISTRY.configure_actix(actix_cfg);
    };

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(LOGGER.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_actix_main)
    })
        .bind(server_url)?
        .run()
        .await
}