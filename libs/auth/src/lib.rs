pub mod access;
pub mod claim;


use headers::{ };

use axum::{
    extract::Request,
    middleware::Next,
    http::{StatusCode ,header},
    response::Response,
    body::Body
};
use axum::extract::Query;

/** 校验身份中间件*/
pub async fn validate_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // 处理校验 token,校验token权限
    let closure = || -> bool {
        // 查找请求头
        return true;
    };

    // 有 token
    if closure(){
        return next.run(request).await
    };
    // 无权限 401
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(axum::body::Body::default())
        .unwrap()
}
