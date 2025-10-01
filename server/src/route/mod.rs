pub mod hello;
pub mod user;

use crate::AppState;
use axum::Router;
pub use hello::*;

// 获取所有路由
pub fn get_routes() -> Router<AppState> {
    let routes = Router::new()
        .merge(user::get_routes())
        .merge(hello::get_routes());
    routes
}
