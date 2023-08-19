mod database;
mod error;
mod handler;
mod service;
use axum::{extract::State, http::HeaderValue, routing::get, Router};
use database::database::{db_connection, DBstate};
use handler::handler::{darazv1, hamro_bazarv1};
use std::env;
use tower_http::cors::CorsLayer;

pub async fn server() {
    let database: DBstate = db_connection().await;
    let app = Router::new()
        .route("/api/daraz", get(darazv1))
        .route("/api/hamrobazar", get(hamro_bazarv1))
        .with_state(database)
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()));

    let server_address = env::var("SERVER_ADDRESS").expect("Server Address not set in environment");
    let server_port = env::var("SERVER_PORT").expect("Server Port not set in environment");
    let server = format!("{}:{}", server_address, server_port);

    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
