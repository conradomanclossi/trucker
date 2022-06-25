use axum::{
    response::Html,
    routing::{delete, get, post, put},
    Router,
};

use crate::modules::user;

async fn hello_world() -> Html<&'static str> {
    Html("<p>Hello, World!!!</p>")
}

pub fn routes() -> Router {
    return Router::new()
        .route("/", get(hello_world))
        .route("/user", get(user::get))
        .route("/user/new", post(user::new))
        .route("/user/all", get(user::all))
        .route("/user/update", put(user::update))
        .route("/user/delete", delete(user::delete));
}
