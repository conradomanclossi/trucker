use crate::{database::establish_connection, diesel::RunQueryDsl, models::User};
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::{query_dsl::methods::FindDsl, PgConnection};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Id {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn user(Json(payload): Json<Id>) -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection: PgConnection = establish_connection();

    let result: Vec<User> = users.find(payload.id).load::<User>(&connection).unwrap();

    return (StatusCode::OK, Json(result));
}

pub async fn update_user(Json(payload): Json<UpdateUser>) -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection: PgConnection = establish_connection();

    let result: Vec<User> = users.find(payload.id).load::<User>(&connection).unwrap();

    let updated_user: User = User {
        id: payload.id,
        name: payload.name,
        email: payload.email,
        password: payload.password,
        created_at: result[0].created_at,
        updated_at: chrono::Utc::now().into(),
    };

    let response = diesel::update(users.find(payload.id))
        .set(&updated_user)
        .execute(&connection);

    match response {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(users.find(payload.id).load::<User>(&connection).unwrap()),
            )
        }
        Err(_) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                Json(users.find(payload.id).load::<User>(&connection).unwrap()),
            )
        }
    }
}

pub async fn delete_user(Json(payload): Json<Id> ) -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection: PgConnection = establish_connection();

    let response = diesel::delete(users.find(payload.id))
        .execute(&connection);

    match response {
        Ok(_) => {return StatusCode::OK},
        Err(_) => {return StatusCode::EXPECTATION_FAILED}
    }

}