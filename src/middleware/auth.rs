use axum::{extract::Request, http::header, middleware::Next, response::Response};
use shiori_jwt::AccessToken;

use crate::errors::{APIError, APIResult};

pub async fn auth_middleware(req: Request, next: Next) -> APIResult<Response> {
    let auth_header = req.headers().get(header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| return APIError::Unauthorized)?,
        None => return Err(APIError::Unauthorized),
    };

    let (scheme, token) = auth_header.split_once(' ').ok_or(APIError::Unauthorized)?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return Err(APIError::Unauthorized);
    }

    // I dont need the whole user for now
    let _user_id = AccessToken::decode(token).map_err(|e| {
        tracing::error!("Failed to decode JWT: {:?}", e);
        APIError::Unauthorized
    })?;

    Ok(next.run(req).await)
}
