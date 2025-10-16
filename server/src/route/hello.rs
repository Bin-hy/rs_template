use crate::AppState;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

// utoipa宏会自动生成路径模块，不需要手动定义

#[cfg(feature = "openapi")]
use crate::route::swagger_ui::ApiDoc;
#[cfg(feature = "openapi")]
use utoipa::OpenApi;
#[cfg(feature = "openapi")]
use utoipa_axum::router::OpenApiRouter;
#[cfg(feature = "openapi")]
use utoipa_axum::routes;
#[cfg(feature = "openapi")]
use utoipa_swagger_ui::SwaggerUi;

#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Deserialize, Serialize)]
pub struct Hello {
    pub msg: String,
}

#[cfg_attr(feature="openapi",utoipa::path(
    get,
    path = "/hello",
    responses((status= 200, description = "你好路由!", body= Hello)),
))]
pub async fn hello_handler() -> Result<Json<Hello>, StatusCode> {
    Ok(Json(Hello {
        msg: "你好！".to_string(),
    }))
}

pub fn get_routes() -> Router<AppState> {
    let routes = Router::new().route("/hello", get(hello_handler));
    routes
}
