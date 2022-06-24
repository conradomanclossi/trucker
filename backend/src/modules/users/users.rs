use crate::{database::establish_connection, diesel::RunQueryDsl, models::User};
use axum::{Json, response::IntoResponse, http::StatusCode};
use diesel::PgConnection;

pub async fn show_users() -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection: PgConnection = establish_connection();

    let results: Vec<User> = users.load::<User>(&connection).unwrap();

    return (StatusCode::OK, Json(results));
}
