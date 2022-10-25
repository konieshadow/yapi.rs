use core::ops::{Deref, DerefMut};
use axum::{extract::{FromRequest, RequestParts}, Json, async_trait};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidateJson<T>(pub T);

#[async_trait]
impl <T, B> FromRequest<B> for ValidateJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<tower::BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let json = Json::<T>::from_request(req).await;
        let Json(value) = json.map_err(|_| Error::BadRequest)?;
        value.validate()?;
        Ok(ValidateJson(value))
    }
}

impl <T> Deref for ValidateJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> DerefMut for ValidateJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <T> From<T> for ValidateJson<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}