use derive_more::Display;
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Display)]
#[diesel(table_name = users)]
#[display(fmt = "name {}, login {}", name, login)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UserMessage {
    pub name: String,
    pub password: String,
}
