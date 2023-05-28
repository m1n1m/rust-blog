use diesel::{RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::db;
use models::schema::users;
use crate::modules::users::users_service;
use models::users::{User, UserMessage};

pub fn find_all() -> Result<Vec<User>, ApiError> {
    let mut conn = db::connection()?;
    let result = users::table.load::<User>(&mut conn)?;
    Ok(result)
}

pub fn find(id: Uuid) -> Result<User, ApiError> {
    let mut conn = db::connection()?;

    let user = users::table
        .filter(users::user_id.eq(id))
        .first(&mut conn)?;

    Ok(user)
}

pub fn create(user: UserMessage) -> Result<User, ApiError> {
    let mut conn = db::connection()?;

    let user = users_service::from(user);
    let user = diesel::insert_into(users::table)
        .values(user)
        .get_result(&mut conn)
        .unwrap();

    Ok(user)
}

pub fn update(id: Uuid, user: UserMessage) -> Result<User, ApiError> {
    let mut conn = db::connection()?;

    let user = diesel::update(users::table)
        .filter(users::user_id.eq(id))
        .set(user)
        .get_result(&mut conn)?;

    Ok(user)
}

pub fn delete(id: Uuid) -> Result<usize, ApiError> {
    let mut conn = db::connection()?;

    let res = diesel::delete(
        users::table
            .filter(users::user_id.eq(id))
    )
        .execute(&mut conn)?;

    Ok(res)
}

fn from(user: UserMessage) -> User {
    User {
        user_id: Uuid::new_v4(),
        name: user.name,
        login: "".to_string(),
        password: user.password,
    }
}
