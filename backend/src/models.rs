use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}
