use derive_more::Display;
use serde::{Deserialize, Serialize};
use diesel::Queryable;

#[derive(Debug, Serialize, Deserialize, Queryable, Display)]
#[diesel(table_name = crate::schema::users)]
#[display(fmt = "user_id: {}, name {}, login {}", user_id, name, login)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
}