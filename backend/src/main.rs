
mod logs;
mod db;
mod modules;
mod config;
mod api_error;
#[cfg(test)]
mod tests;

use actix_web::HttpServer;
use actix_web::App;
use actix_web::middleware::Logger;
use slog::info;
use crate::logs::LOGGER;
use crate::modules::users;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    config::init();
    db::init();

    info!(LOGGER, "Starting the server at {}", config::get_config().server_url);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(users::routes::init_routes)
    })
        .bind(config::get_config().server_url)?
        .run()
        .await
}
