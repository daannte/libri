use axum::{
    extract::{FromRequestParts, Path, Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use shiori_database::{
    models::{ApiToken, User},
    token::HashedToken,
};
use shiori_jwt::AccessToken;

use crate::{
    config::state::AppState,
    errors::{AppResult, BoxedAppError, unauthorized},
};

pub async fn auth_middleware(
    State(app): State<AppState>,
    mut req: Request,
    next: Next,
) -> AppResult<Response> {
    let jar = CookieJar::from_headers(req.headers());
    let authorization = req.headers().get(header::AUTHORIZATION);

    if let Some(auth_header) = authorization {
        let auth_header = auth_header.to_str().map_err(|_| {
            unauthorized("Invalid `Authorization` header: Found non-ASCII characters")
        })?;

        let (scheme, token) = auth_header.split_once(' ').unwrap_or(("", auth_header));

        if !scheme.eq_ignore_ascii_case("bearer") {
            return Err(unauthorized(format!(
                "Invalid `Authorization` header: Found unexpected authentication scheme: `{scheme}`"
            )));
        }

        let token = HashedToken::parse(token).map_err(|_| unauthorized("Invalid API token"))?;

        let user_id = update_last_used_at(app, &token.hash).await?;

        req.extensions_mut().insert(AuthContext::Token(user_id));
        return Ok(next.run(req).await);
    }

    if let Some(cookie) = jar.get("access_token") {
        let user_id_str = AccessToken::decode(cookie.value())
            .map_err(|_| unauthorized("Invalid access token"))?;

        let user_id = user_id_str
            .parse::<i32>()
            .map_err(|_| unauthorized("Invalid access token"))?;

        req.extensions_mut().insert(AuthContext::Cookie(user_id));
        return Ok(next.run(req).await);
    }

    Err(unauthorized("Unauthorized"))
}

pub async fn url_auth_middleware(
    State(app): State<AppState>,
    Path(api_token): Path<String>,
    mut req: Request,
    next: Next,
) -> AppResult<Response> {
    let token = HashedToken::parse(&api_token).map_err(|_| unauthorized("Invalid API token"))?;
    let user_id = update_last_used_at(app, &token.hash).await?;

    req.extensions_mut().insert(AuthContext::Token(user_id));

    Ok(next.run(req).await)
}

#[derive(Clone)]
pub enum AuthContext {
    Cookie(i32),
    Token(i32),
}

pub enum Auth {
    Cookie(User),
    Token(User),
}

impl Auth {
    pub fn is_token(&self) -> bool {
        matches!(self, Auth::Token(_))
    }

    pub fn user(&self) -> &User {
        match self {
            Auth::Token(user) | Auth::Cookie(user) => user,
        }
    }
}

pub struct AuthUser(pub Auth);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = BoxedAppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .extensions
            .get::<AuthContext>()
            .ok_or_else(|| unauthorized("Unauthorized"))?;

        let mut conn = state.db().await?;

        let user_id = match auth {
            AuthContext::Cookie(id) => *id,
            AuthContext::Token(id) => *id,
        };

        let user = User::find(&mut conn, user_id).await?;

        let auth = match auth {
            AuthContext::Cookie(_) => Auth::Cookie(user),
            AuthContext::Token(_) => Auth::Token(user),
        };

        Ok(AuthUser(auth))
    }
}

async fn update_last_used_at(app: AppState, token: &Vec<u8>) -> AppResult<i32> {
    let mut conn = app.db().await?;
    let api_token = ApiToken::find_by_hash(&mut conn, token)
        .await
        .map_err(|_| unauthorized("Invalid API token"))?;

    if let Err(e) = api_token.update_last_used(&mut conn).await {
        tracing::error!(
            api_token = api_token.key_id,
            error = %e,
            "Failed to update last used time"
        );
    };

    Ok(api_token.user_id)
}
