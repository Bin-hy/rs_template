use axum::Json;
use axum::http::StatusCode;
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
