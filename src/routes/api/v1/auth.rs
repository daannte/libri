use axum::{Json, extract::State};
use serde::Deserialize;
use shiori_api_types::EncodableUser;
use utoipa_axum::{router::OpenApiRouter, routes};

use shiori_database::models::{NewUser, User};

use crate::{
    auth::hash_password,
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

/// Login
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully logged in"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login() {}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    /// Username the user wants to register with.
    #[schema(examples("Reaper"))]
    username: String,

    /// Password for the user.
    #[schema(examples("supercoolpass"))]
    password: String,
}

/// Register
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = inline(RegisterRequest),
    responses(
        (status = 200, description = "Successfully registered", body = inline(EncodableUser)),
        (status = 400, description = "Bad request payload"),
        (status = 409, description = "Username already taken"),
        (status = 500, description = "Internal server error")
    )
)]
async fn register(
    State(app): State<AppState>,
    Json(body): Json<RegisterRequest>,
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
