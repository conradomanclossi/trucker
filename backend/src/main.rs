#[macro_use]
extern crate diesel;
extern crate dotenv;

use axum::{Router, Server};
use std::net::SocketAddr;

mod database;
mod models;
mod modules;
mod routes;
mod schema;
use routes::routes;

#[tokio::main]
async fn main() {
    // Routes
    let app: Router = routes();

    // run it
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
