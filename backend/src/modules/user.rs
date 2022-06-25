use crate::{
    database::establish_connection,
    diesel::RunQueryDsl,
    models::{Id, NewUser, UpdateUser, User},
    schema::users::dsl::*,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::{query_dsl::methods::FindDsl, PgConnection};

pub async fn get(Json(payload): Json<Id>) -> impl IntoResponse {
    let connection: PgConnection = establish_connection();

    let result: Vec<User> = users.find(payload.id).load::<User>(&connection).unwrap();

    return (StatusCode::OK, Json(result));
}

pub async fn new(Json(payload): Json<NewUser>) -> impl IntoResponse {
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

pub async fn all() -> impl IntoResponse {
    let connection: PgConnection = establish_connection();

    let results: Vec<User> = users.load::<User>(&connection).unwrap();

    return (StatusCode::OK, Json(results));
}

pub async fn update(Json(payload): Json<UpdateUser>) -> impl IntoResponse {
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

pub async fn delete(Json(payload): Json<Id>) -> impl IntoResponse {
    let connection: PgConnection = establish_connection();

    let response = diesel::delete(users.find(payload.id)).execute(&connection);

    match response {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::EXPECTATION_FAILED,
    }
}
