use std::env;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use log::info;
use serde::{Deserialize, Serialize};
use route::hello;
use crate::config::Config;
use crate::route::hello_handler;

pub mod config;
pub mod route;
pub mod api;
pub mod result;
pub mod error;

// 运行 函数
fn generate_router() -> Router {
    let router = Router::new();
    router
}

pub async fn serve(cfg:Config, listener: TcpListener){
    info!("Server listening on {}", listener.local_addr().unwrap());

    let hello_router = Router::new().route("/",
        get(hello_handler)
    );
    let app = Router::new().nest("/hello",hello_router);
    axum::serve(listener, app).await.unwrap();
}