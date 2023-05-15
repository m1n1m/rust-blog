
mod handlers;
mod models;
mod schema;
mod logs;

use actix_web::HttpServer;
use actix_web::App;
use actix_web::middleware::Logger;
use actix_web::web;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use slog::{info};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let log = logs::configure_log();

    // create db connection pool
    info!(log,
        "Creating DB connection pool"
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    info!(log,
        "Starting the server at http://127.0.0.1:8080/"
    );

    // Start http server
    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            .app_data(web::Data::new(log.clone()))
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}