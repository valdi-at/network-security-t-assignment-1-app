#![allow(unused)]
use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::RwLock;

mod api_response;
mod handler {
    pub mod hello;
}

mod app_state;

use crate::{app_state::AppState, handler::hello::hello_handler};

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(AppState::default()));
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
