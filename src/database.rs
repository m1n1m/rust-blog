

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use slog::info;
use crate::logs;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        info!(logs::LOGGER, "Initializing database");
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_pool(&self) -> DBPool
    {
        self.pool.clone()
    }
}