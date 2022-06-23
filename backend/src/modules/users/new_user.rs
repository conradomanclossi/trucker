use crate::{database::establish_connection, diesel::RunQueryDsl, models::NewUser};
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::PgConnection;

pub async fn new(Json(payload): Json<NewUser>) -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection: PgConnection = establish_connection();

    let new_user: NewUser = NewUser {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let response = diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection);

    match response {
        Ok(_) => return (StatusCode::CREATED, Json(new_user)),
        Err(_) => return (StatusCode::EXPECTATION_FAILED, Json(new_user)),
    }
}
