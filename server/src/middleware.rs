use axum::http::{HeaderValue, StatusCode};
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn log_middleware(req: Request, next: Next) -> Response {
    println!("Incoming request: {} {}", req.method(), req.uri());
    let resp = next.run(req).await;
    resp
}

// 身份校验
pub async fn auth(req: Request, next: Next) -> Response {
    // 检查获取请求头的 token
    let token = match auth::get_token(&req.headers()) {
        Some(token) => {
            println!("Token found: {}", token);
        }
        None => {}
    };
    println!("authorization: {:?}", token);
    let resp = next.run(req).await;
    resp
}

// 身份校验
mod auth {
    use axum::http::HeaderMap;
    use axum::http::header::AUTHORIZATION;

    pub(crate) fn get_token(headers: &HeaderMap) -> Option<String> {
        Some(headers.get(AUTHORIZATION)?.to_str().ok()?.to_string())
    }
}
