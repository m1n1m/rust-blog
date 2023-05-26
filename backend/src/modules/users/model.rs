use derive_more::Display;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::db;
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Display)]
#[diesel(table_name = crate::schema::users)]
#[display(fmt = "name {}, login {}", name, login)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct UserMessage {
    pub name: String,
    pub password: String,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let users = users::table.load::<User>(&mut conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::user_id.eq(id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn)
            .unwrap();

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
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
}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            user_id: Uuid::new_v4(),
            name: user.name,
            login: "".to_string(),
            password: user.password,
        }
    }
}
