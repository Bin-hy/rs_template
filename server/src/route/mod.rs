pub mod hello;
pub mod user;
pub mod swagger_ui;

use crate::AppState;
use axum::Router;
pub use hello::*;


// 获取所有路由
pub fn get_routes() -> Router<AppState> {
    let mut routes = Router::new()
        .merge(user::get_routes())
        .merge(hello::get_routes());
    #[cfg(feature = "openapi")]
    {
        routes = routes.merge(get_routes_docs());
    }
    routes
}

#[cfg(feature = "openapi")]
pub fn get_routes_docs() -> Router<AppState> {
    use swagger_ui::ApiDoc;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    let openapi = ApiDoc::openapi();
    SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", openapi)
        .into()

}
