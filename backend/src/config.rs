use lazy_static::lazy_static;

#[cfg(not(test))]
fn read_dotenv() {
    dotenv::dotenv().ok();
}

#[cfg(test)]
fn read_dotenv() {
    dotenv::from_filename(".env.test").ok();
}

pub fn read_env() {
    read_dotenv();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "full");
}

pub struct AppConfig {
    pub db_url: &'static str,
    pub server_url: &'static str
}

lazy_static! {
        pub static ref APP_CONFIG: AppConfig = {
            read_env();
            env_logger::init();
            AppConfig {
                db_url: Box::leak(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set").into_boxed_str()),
                server_url: Box::leak(std::env::var("SERVER_URL").expect("SERVER_URL must be set").into_boxed_str()),
            }
       };
    }

pub fn init() {
    lazy_static::initialize(&APP_CONFIG);
}

pub fn get_config() -> &'static APP_CONFIG {
    &APP_CONFIG
}