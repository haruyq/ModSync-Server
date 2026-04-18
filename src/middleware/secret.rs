use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::config;

pub async fn api_secret(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let config = config::load_config();

    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok());

    if api_key != Some(&config.secret) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}