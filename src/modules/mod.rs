pub mod users;

use actix_web::web::ServiceConfig;
use crate::modules::users::UsersModule;


pub trait AppModule {
    fn configure(&self, actix_cfg: &mut ServiceConfig);
}

pub struct ModuleRegistry;

impl ModuleRegistry {
    pub fn configure_actix(&self, actix_cfg: &mut ServiceConfig) {
        static USERS_MODULE: UsersModule = UsersModule {};
        USERS_MODULE.configure_actix(actix_cfg);
    }
}