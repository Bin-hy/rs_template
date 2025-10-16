use crate::AppState;
use crate::error::AppError;
use anyhow::Error;
use auth::access::Token;
use auth::claim::Claim;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router, extract::Query};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use utoipa::OpenApi;
#[cfg(feature = "openapi")]
use utoipa_axum::router::OpenApiRouter;
#[cfg(feature = "openapi")]
use utoipa_axum::routes;
#[cfg(feature = "openapi")]
use utoipa_swagger_ui::SwaggerUi;

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

#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Deserialize, Serialize, Clone, Debug)]
struct UpdateQuery {
    id: String,
    limit: Option<u32>,
}

#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

/** 登陆请求 */
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Clone, Serialize, Deserialize)]
struct UserLogin {
    username: String,
    password: String,
    // code : String,
}

type TOKEN = String;

#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Clone, Serialize, Deserialize)]
struct UserLoginResponse {
    token: TOKEN,
}

#[cfg_attr(feature="openapi",utoipa::path(
    get,
    path = "/update_user_by_id",
    params(
        ("id" = u64, Query, description = "用户 ID"),
        ("name" = Option<String>, Query, description = "新用户名")
    ),
    responses((status= 200, description = "根据 ID 更新用户", body= String)),
    tag = "User 管理"
))]
pub async fn update_user_by_id(
    Query(params): Query<UpdateQuery>,
) -> Result<impl IntoResponse, AppError> {
    let id = params.id;
    Ok(format!("update_user_by_id: {}", id).into_response())
}

#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Deserialize, Serialize, Clone, Debug)]
struct DeleteQuery {
    id: u64,
}

#[cfg_attr(feature="openapi",utoipa::path(
    delete,
    path = "/delete_user_by_id",
    responses((status= 200, description = "根据ID删除用户", body= String)),
    tag = "User 管理"
))]
pub async fn delete_user_by_id(
    Query(params): Query<DeleteQuery>,
) -> Result<impl IntoResponse, AppError> {
    let id = params.id;
    Ok(format!("update_user_by_id: {}", id).into_response())
}

#[cfg_attr(feature="openapi",utoipa::path(
    post,
    request_body = User,
    path = "/login_check",
    responses((status= 200, description = "登陆校验", body= String)),
    tag = "User 管理"
))]
pub async fn login_check_user(State(app_state): State<AppState>, Form(user): Form<User>) -> String {
    format!(
        "url: {}, 登录用户: {}, 密码: {}",
        app_state.config.database.url, user.id, user.name
    )
}

#[cfg_attr(feature="openapi",utoipa::path(
    post,
    request_body = User,
    path = "/add_user",
    responses((status= 200, description = "添加用户", body= User)),
    tag = "User 管理"
))]
pub async fn add_user(Json(user): Json<User>) -> Json<User> {
    Json(user)
}

#[cfg_attr(feature="openapi",utoipa::path(
    get,
    params (
        ("user_id" = u64, Path, description = "用户ID")
    ),
    path = "/get_user_by_id",
    responses((status= 200, description = "获取用户根据Id", body= User)),
    tag = "User 管理"
))]
pub async fn get_user_by_id(Path(user_id): Path<u64>) -> Result<Json<User>, AppError> {
    Ok(Json(User {
        id: user_id,
        name: "xhy".to_string(),
    }))
}

#[cfg_attr(feature="openapi",utoipa::path(
    get,
    path = "/try_err",
    responses((status= 200, description = "错误示例", body= String)),
    tag = "User 管理"
))]
pub async fn try_response_with_app_error() -> Result<String, AppError> {
    Err(AppError::InternalServerError(anyhow::Error::msg(
        "Internal server error",
    )))
}

async fn not_found() -> Result<impl IntoResponse, AppError> {
    Ok("404: Not Found".into_response())
}

/** POST 登陆*/
#[cfg_attr(feature="openapi",utoipa::path(
    post,
    request_body = UserLogin,
    path = "/login",
    responses((status= 200, description = "登陆", body= String)),
    tag = "User 管理"
))]
pub async fn login(Json(payload): Json<UserLogin>) -> crate::result::Result<Response> {
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
    Ok(Response::builder()
        .status(http::StatusCode::OK)
        .body("Don't permission".into())
        .unwrap())
}

#[cfg(feature="openapi")]
#[derive(OpenApi)]
#[openapi(

    paths(
        update_user_by_id,
        delete_user_by_id,
        get_user_by_id,
        add_user,
        login_check_user,
        login,
        try_response_with_app_error
    ),
    tags ((name = "User 管理", description = "普通用户相关接口"),)
)]
pub struct UserApi;
