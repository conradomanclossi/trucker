use axum::{response::Html, response::Json, routing::get, routing::post, Router, Server};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize, Serialize)]
struct CreateUser {
    name: String,
}

async fn handler() -> Html<&'static str> {
    Html("<p>Hello, World!!!</p>")
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<CreateUser> {
    Json(CreateUser { name: payload.name })
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/user", post(create_user));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
