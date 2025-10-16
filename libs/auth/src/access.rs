// 检验 token 的中间件
use crate::claim::Claim;
use axum::http::{HeaderMap, header::AUTHORIZATION};
use axum::{extract::Request, middleware::Next, response::Response};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

/** Token*/
#[derive(Debug, Clone)]
pub struct Token(pub String);

/** 生成一个token*/
pub fn generate_token(claim: Claim, secret: &str) -> String {
    // 生成 token
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("JWT encode failed");

    token
}

/** 检验token的有效性 */
pub fn check_token(token: String, secret: &str) -> bool {
    // 检查 解密
    decode::<Claim>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .is_ok()
}

/** 身份校验 权限中间件*/
pub async fn access_middleware(req: Request, next: Next) -> Response {
    // 检查获取请求头的 token
    let token = match get_token(&req.headers()) {
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
pub(crate) fn get_token(headers: &HeaderMap) -> Option<String> {
    // let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    headers
        .get(AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
    // value.strip_prefix("Bearer ")?
}
