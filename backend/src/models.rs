use crate::schema::users;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
