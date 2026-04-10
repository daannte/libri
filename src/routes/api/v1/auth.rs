use axum::{Json, extract::State};
use serde::Deserialize;
use shiori_api_types::EncodableUser;
use shiori_jwt::create_jwt_tokens;
use utoipa_axum::{router::OpenApiRouter, routes};

use shiori_database::models::{NewUser, User};

use crate::{
    auth::{hash_password, verify_password},
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(register))
        .routes(routes!(refresh_token))
        .routes(routes!(logout))
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AuthRequest {
    /// Username of the account.
    #[schema(examples("Reaper"))]
    username: String,

    /// Password associated with the account.
    #[schema(examples("supercoolpass"))]
    password: String,
}

/// Login
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = inline(AuthRequest),
    responses(
        (status = 200, description = "Successfully logged in"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login(State(app): State<AppState>, Json(body): Json<AuthRequest>) -> APIResult<()> {
    let mut conn = app.db().await?;

    let user = User::find_by_username(&mut conn, &body.username)
        .await?
        .ok_or_else(|| APIError::Unauthorized)?;

    let _valid = verify_password(&user.hashed_password, &body.password)
        .map_err(|_| APIError::Unauthorized)?;

    let _tokens = create_jwt_tokens(&user.id).await;

    Ok(())
}

/// Register
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = inline(AuthRequest),
    responses(
        (status = 200, description = "Successfully registered", body = inline(EncodableUser)),
        (status = 400, description = "Bad request payload"),
        (status = 409, description = "Username already taken"),
        (status = 500, description = "Internal server error")
    )
)]
async fn register(
    State(app): State<AppState>,
    Json(body): Json<AuthRequest>,
) -> APIResult<Json<EncodableUser>> {
    let mut conn = app.db().await?;

    let has_users = User::count(&mut conn).await? > 0;
    let is_server_owner = !has_users;

    if body.password.len() < 8 {
        return Err(APIError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let hash = hash_password(&body.password)?;

    let new_user = NewUser {
        username: &body.username,
        hashed_password: &hash,
        is_server_owner,
    };

    let user = new_user.insert(&mut conn).await?;

    Ok(Json(user.into()))
}

/// Refresh JWT token
#[utoipa::path(
    post,
    path = "/auth/refresh-token",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully refreshed JWT token"),
        (status = 500, description = "Internal server error")
    )
)]
async fn refresh_token() {}

/// Logout
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully logged out"),
        (status = 500, description = "Internal server error")
    )
)]
async fn logout() {}
