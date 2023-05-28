use diesel::{PgConnection};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use slog::info;
use crate::{logs};
use crate::api_error::ApiError;
use crate::config::get_config;


pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(conn: &mut PgConnection) {
    info!(logs::LOGGER, "Running migrations");
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

lazy_static! {
        static ref DB_POOL: DBPool = {
            info!(logs::LOGGER, "Initializing database");
            crate::config::init();
            info!(logs::LOGGER, "Database url : {}", get_config().db_url);
            let manager = ConnectionManager::<PgConnection>::new(get_config().db_url);
            let pool_size = match cfg!(test) {
               true => 1,
               false => 10,
            };
            r2d2::Builder::new().max_size(pool_size).build(manager).expect("Failed to create db pool")
       };
    }

pub fn init() {
    lazy_static::initialize(&DB_POOL);
    let mut conn = connection().expect("Failed to get db connection");
    if cfg!(test) {
    } else {
        run_migration(&mut conn);
    }
}

#[cfg(test)]
pub fn before_test() {

    // let mut conn = connection().expect("Failed to get db connection");
    // // conn.batch_execute("DROP SCHEMA IF EXISTS public CASCADE").unwrap();
    // conn.batch_execute("CREATE SCHEMA IF NOT EXISTS public").unwrap();
    // run_migration(&mut conn);

    // // let mut client = DB_POOL.get().unwrap();
    // client.batch_execute("DROP SCHEMA IF EXISTS public CASCADE");
    //
    // thread::spawn(move || {
    //     let mut client = DB_POOL.get().unwrap();
    //     client.batch_execute("INSERT INTO foo (bar) VALUES ($1)", &[&i]).unwrap();
    // });

    // let result = tokio_postgres::connect("host=localhost user=postgres", NoTls).await;

    // futures::executor::block_on(async {
    //     let result = tokio_postgres::connect("host=localhost user=postgres", NoTls).await;
    //     result
    // })

    // let (client, connection) = Config::new()
    //     .host("localhost")
    //     .user("admin")
    //     .port(5432)
    //     .password("secret_password")
    //     .dbname("admin")
    //     .connect(NoTls)
    //     .await
    //     .unwrap();

    // let (client, connection) =
    //     tokio_postgres::connect("host=localhost user=postgres", NoTls).await;

    // let mut client = Client::connect(get_config().db_url, NoTls).unwrap();
    // client.execute("DROP SCHEMA IF EXISTS public CASCADE", &[]).unwrap();
    // client.execute("CREATE SCHEMA public", &[]).unwrap();
}

pub fn connection() -> Result<DbConnection, ApiError> {
    DB_POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}