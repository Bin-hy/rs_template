use crate::config::Config;
#[cfg(feature = "openapi")]
use route::swagger_ui::ApiDoc;
use auth::access;
use axum::Router;
use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use log::info;
use route::hello;
use serde::{Deserialize, Serialize};
use service::database::DatabaseService;
use std::cell::Cell;
use std::env;
use std::sync::Arc;
use http::Uri;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info_span;
#[cfg(feature = "openapi")]
use utoipa::OpenApi;
#[cfg(feature = "openapi")]
use utoipa_swagger_ui::SwaggerUi;

pub mod api;
pub mod config;
pub mod error;
mod middleware;
pub mod result;
pub mod route;
pub mod service;

#[cfg(feature = "webui")]
#[derive(rust_embed::RustEmbed)]
#[folder = "../assets/server/"]
struct Assets; // 配置静态资源路径

#[cfg(feature = "webui")]
async fn static_web_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/');
    if path.is_empty() {
        path = "index.html";
    }
    match Assets::get(path){
        Some(webContent) =>{
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(http::header::CONTENT_TYPE, mime.as_ref())], webContent.data).into_response()
        }
        None =>(http::StatusCode::NOT_FOUND, "not found").into_response(),
    }
}
#[cfg(not(feature = "webui"))]
async fn static_web_handler(_:Uri) -> impl IntoResponse {
    (http::StatusCode::NOT_FOUND, "feature webui not enable. \n if you need, please add '--features webui' ")
}
// 运行 函数
fn generate_router(cfg: &Config) -> Router<AppState> {
    let mut router = Router::new()
        .merge(route::get_routes())
        .layer(axum::middleware::from_fn(middleware::log_middleware)) // 日志
        // .layer(axum::middleware::from_fn(middleware::auth))
        .layer(axum::middleware::from_fn(access::access_middleware)) // 鉴权中间件
        .layer(tower_http::trace::TraceLayer::new_for_http()) //
        .layer(if cfg.http.cors {
            // 配置跨域
            CorsLayer::permissive() // Allow 均允许
        } else {
            CorsLayer::new() // new函数，默认初始值 均为 不允许 NO
        });
    // .with_state(app_state);

    router
}

pub async fn serve(cfg: Config, listener: TcpListener) {
    info!("Server listening on {}", listener.local_addr().unwrap());
    // 初始化 数据库连接
    let database_service = DatabaseService::new(&cfg.database)
        .await
        .expect("Failed to initialize database connection");

    let app_state = AppState {
        config: cfg.clone(),
        database: database_service,
    };

    let app = Router::new()
        .merge(generate_router(&cfg))
        .with_state(app_state)
        .fallback(static_web_handler);

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    config: Config,
    database: DatabaseService,
}
