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

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    iat: i64,
    exp: i64,
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    iat: i64,
    exp: i64,
    sub: String,
    jti: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct JwtTokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl JwtTokenPair {
    pub fn new(user_id: i32) -> Result<JwtTokenPair, jsonwebtoken::errors::Error> {
        Ok(Self {
            access_token: AccessToken::new(user_id)?,
            refresh_token: RefreshToken::new(user_id)?,
        })
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub(crate) fn new(user_id: i32) -> Result<AccessToken, jsonwebtoken::errors::Error> {
        let (iat, exp, exp_dt) = jwt_times(Duration::minutes(15));

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

        Ok(Self {
            token,
            expires_at: exp_dt,
        })
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct RefreshToken {
    pub token: String,
    #[serde(skip_serializing)]
    pub jti: String,
    pub expires_at: DateTime<Utc>,
}

impl RefreshToken {
    pub(crate) fn new(user_id: i32) -> Result<RefreshToken, jsonwebtoken::errors::Error> {
        let (iat, exp, exp_dt) = jwt_times(Duration::days(7));
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

        Ok(Self {
            token,
            jti,
            expires_at: exp_dt,
        })
    }
}

fn jwt_times(duration: Duration) -> (i64, i64, DateTime<Utc>) {
    let now = Utc::now();
    let exp_dt = now + duration;

    (now.timestamp(), exp_dt.timestamp(), exp_dt)
}
