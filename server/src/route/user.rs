use crate::AppState;
use crate::error::AppError;
use anyhow::Error;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router, extract::Query};
use serde::{Deserialize, Serialize};

// 获取当前路由
pub fn get_routes() -> Router<AppState> {
    let routes = Router::new()
        .route(
            "/update_user_by_id",
            get(update_user_by_id).post(update_user_by_id),
        )
        .route("/delete_user_by_id", delete(delete_user_by_id))
        .route("/get_user_by_id", get(get_user_by_id))
        .route("/add_user", post(add_user))
        .route("/login_check", post(login_check_user))
        .route("/try_err", get(try_response_with_app_error))
        .fallback(not_found);
    let res_routes = Router::new().nest("/users", routes);
    res_routes
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct UpdateQuery {
    id: String,
    limit: Option<u32>,
}
#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn update_user_by_id(
    Query(params): Query<UpdateQuery>,
) -> Result<impl IntoResponse, AppError> {
    let id = params.id;
    Ok(format!("update_user_by_id: {}", id).into_response())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DeleteQuery {
    id: u64,
}

async fn delete_user_by_id(
    Query(params): Query<DeleteQuery>,
) -> Result<impl IntoResponse, AppError> {
    let id = params.id;
    Ok(format!("update_user_by_id: {}", id).into_response())
}

async fn login_check_user(State(app_state): State<AppState>, Form(user): Form<User>) -> String {
    format!(
        "url: {}, 登录用户: {}, 密码: {}",
        app_state.config.database.url, user.id, user.name
    )
}

async fn add_user(Json(user): Json<User>) -> Json<User> {
    Json(user)
}

async fn get_user_by_id(Path(user_id): Path<u64>) -> Result<Json<User>, AppError> {
    Ok(Json(User {
        id: user_id,
        name: "xhy".to_string(),
    }))
}

async fn try_response_with_app_error() -> Result<String, AppError> {
    Err(AppError::InternalServerError(anyhow::Error::msg(
        "Internal server error",
    )))
}

async fn not_found() -> Result<impl IntoResponse, AppError> {
    Ok("404: Not Found".into_response())
}
