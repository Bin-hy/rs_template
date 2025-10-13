use crate::AppState;
use crate::error::AppError;
use anyhow::Error;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router, extract::Query};
use serde::{Deserialize, Serialize};
use auth::access::Token;
use auth::claim::{Claim};
use http::{StatusCode};
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
        .route("/login", post(login))
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

/** 登陆请求 */
#[derive(Clone, Serialize, Deserialize)]
struct UserLogin{
    username: String,
    password: String,
    // code : String,
}

type TOKEN = String;
#[derive(Clone, Serialize, Deserialize)]
struct UserLoginResponse{
    token: TOKEN
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

/** POST 登陆*/
async fn login(Json(payload): Json<UserLogin>) -> crate::result::Result<Response> {
    // 检验用户名和密码
    payload.username;
    payload.password;

    // Ok(
    //     Response::builder()
    //         .status(StatusCode::OK)
    //         .body("".into_response())
    // )

    // Ok(Response::builder()
    //     .status(StatusCode::NO_CONTENT)
    //     .body("".to_string())?
    // )

    //  生成 token 返回
    Ok( Response::builder()
        .status(http::StatusCode::OK)
        .body("Don't permission".into())
        .unwrap()
    )
}

