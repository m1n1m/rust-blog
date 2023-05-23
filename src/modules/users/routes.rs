use actix_web::{HttpResponse, web};
use actix_web::web::{ServiceConfig};
use log::info;
use serde_json::json;
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::modules::users::model::{User, UserMessage};

#[actix_web::get("/users")]
pub async fn find_all() -> Result<HttpResponse, ApiError> {
    info!("find_all call");
    let all_users = User::find_all()?;
    Ok(HttpResponse::Ok().json(all_users))
}

#[actix_web::get("/users/{id}")]
pub async fn find_by_id(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    info!("find_by_id, id = {}", id);
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::post("/users")]
async fn create(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::put("/users/{id}")]
async fn update(id: web::Path<Uuid>, user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = User::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}