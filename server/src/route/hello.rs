use crate::AppState;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Hello {
    pub msg: String,
}

pub async fn hello_handler() -> Result<Json<Hello>, StatusCode> {
    Ok(Json(Hello {
        msg: "你好！".to_string(),
    }))
}

pub fn get_routes() -> Router<AppState> {
    let routes = Router::new().route("/hello", get(hello_handler));
    routes
}
