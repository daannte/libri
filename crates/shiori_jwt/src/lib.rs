use base64::{Engine, engine::general_purpose};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn generate_secret() -> String {
    let mut bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}

static ACCESS_TOKEN_SECRET: Lazy<String> = Lazy::new(generate_secret);
static REFRESH_TOKEN_SECRET: Lazy<String> = Lazy::new(generate_secret);

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct JwtTokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    iat: usize,
    exp: usize,
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    iat: usize,
    exp: usize,
    sub: String,
    jti: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct RefreshToken {
    pub token: String,
    #[serde(skip_serializing)]
    pub jti: String,
    pub expires_at: DateTime<Utc>,
}

pub async fn create_jwt_tokens(user_id: i32) -> Result<JwtTokenPair, jsonwebtoken::errors::Error> {
    let access_token = create_access_token(user_id)?;
    let refresh_token = create_refresh_token(user_id)?;

    Ok(JwtTokenPair {
        access_token: access_token,
        refresh_token: refresh_token,
    })
}

fn create_access_token(user_id: i32) -> Result<AccessToken, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_dt = now + Duration::minutes(15);
    let exp = exp_dt.timestamp() as usize;

    let claims = AccessTokenClaims {
        iat,
        exp,
        sub: user_id.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ACCESS_TOKEN_SECRET.as_bytes()),
    )?;

    Ok(AccessToken {
        token,
        expires_at: exp_dt,
    })
}

fn create_refresh_token(user_id: i32) -> Result<RefreshToken, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_dt = now + Duration::days(7);
    let exp = exp_dt.timestamp() as usize;
    let jti = Uuid::new_v4().to_string();

    let claims = RefreshTokenClaims {
        iat,
        exp,
        sub: user_id.to_string(),
        jti: jti.clone(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(REFRESH_TOKEN_SECRET.as_bytes()),
    )?;

    Ok(RefreshToken {
        token,
        jti,
        expires_at: exp_dt,
    })
}
