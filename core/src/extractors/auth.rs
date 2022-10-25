use axum::{http::{HeaderValue, header}, async_trait, extract::{FromRequest, RequestParts}, Extension};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use time::OffsetDateTime;

use crate::{Result, error::Error, Context};

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);
const SCHEME_PREFIX: &str = "Token ";

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: u32,
}

impl AuthUser {
    pub fn new(user_id: u32) -> Self {
        Self { user_id }
    }
}

#[derive(Debug, Clone)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthUserClaims {
    user_id: u32,
    exp: i64,
}

impl AuthUser {
    pub fn to_jwt(&self, hmac_key: &str) -> String {
        let hmac = Hmac::<Sha256>::new_from_slice(hmac_key.as_bytes())
            .expect("HMAC can take key of any size");

        AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    fn from_authorization(hamc_key: &str, auth_header: &HeaderValue) -> Result<Self> {
        let auth_header = auth_header.to_str().map_err(|_| Error::Unauthorized)?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!("authorization header is using the wrong schema: {:?}", auth_header);
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt = jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
            log::debug!("failed to parse authorization header {:?}: {}", auth_header, e);
            Error::Unauthorized
        })?;

        let hmac = Hmac::<Sha256>::new_from_slice(hamc_key.as_bytes())
            .expect("HMAC can take key of any size");
        
        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            log::debug!("failed to verify jwt: {}", e);
            Error::Unauthorized
        })?;

        let (_, claims) = jwt.into();

        Ok(Self {
            user_id: claims.user_id
        })
    }
}

impl MaybeAuthUser {
    pub fn user_id(&self) -> Option<u32> {
        self.0.as_ref().map(|a| a.user_id)
    }
}

#[async_trait]
impl <B> FromRequest<B> for AuthUser
where
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<tower::BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let ctx: Extension<Context> = Extension::from_request(req)
            .await
            .expect("context was not added as extensions");

        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&ctx.config.hmac_key, auth_header)
    }
}