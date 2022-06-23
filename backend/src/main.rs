#[macro_use]
extern crate diesel;
extern crate dotenv;

use axum::{response::Html, routing::get, routing::post, Router, Server};
use std::net::SocketAddr;

mod database;
mod models;
mod modules;
mod schema;
use modules::users::new_user::new;

async fn handler() -> Html<&'static str> {
    Html("<p>Hello, World!!!</p>")
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app: Router = Router::new()
        .route("/", get(handler))
        .route("/new_user", post(new));

    // run it
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
