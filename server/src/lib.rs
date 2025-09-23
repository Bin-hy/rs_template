use crate::config::Config;
use crate::route::hello_handler;
use axum::Router;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use log::info;
use route::hello;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::net::TcpListener;

pub mod api;
pub mod config;
pub mod error;
pub mod result;
pub mod route;

// 运行 函数
fn generate_router() -> Router {
    let router = Router::new();
    router
}

pub async fn serve(cfg: Config, listener: TcpListener) {
    info!("Server listening on {}", listener.local_addr().unwrap());

    let hello_router = Router::new().route("/", get(hello_handler));
    let app = Router::new().nest("/hello", hello_router);
    axum::serve(listener, app).await.unwrap();
}
