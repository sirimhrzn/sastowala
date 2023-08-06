mod database;
mod error;
mod handler;
mod service;
use axum::{routing::get, Router};
use database::database::{db_connection, DBstate};
use handler::handler::darazv1;
use std::env;

pub async fn server() {
    let database: DBstate = db_connection().await;
    let app = Router::new()
        .route("/api/daraz", get(darazv1))
        .with_state(database);

    let server_address = env::var("SERVER_ADDRESS").expect("Server Address not set in environment");
    let server_port = env::var("SERVER_PORT").expect("Server Port not set in environment");
    let server = format!("{}:{}", server_address, server_port);

    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
