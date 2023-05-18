use actix_web::{HttpResponse, Responder, web};
use actix_web::web::{ServiceConfig, Data};
use diesel::RunQueryDsl;
use slog::{info, Logger};
use crate::database::DBPool;
use crate::models::User;
use crate::schema::users::dsl::users;

pub struct UsersModule;

impl UsersModule {
    pub fn configure_actix(&self, actix_cfg: &mut ServiceConfig) {
        actix_cfg.service(get_users);
    }
}

#[actix_web::get("/users")]
pub async fn get_users(db: Data<DBPool>, log: Data<Logger>) -> impl Responder {
    info!(log, "get_users call");

    web::block(move || get_all_users(db))
        .await?
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(actix_web::error::ErrorInternalServerError)
}

fn get_all_users(pool: Data<DBPool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}