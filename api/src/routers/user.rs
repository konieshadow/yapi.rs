use axum::{Router, routing::post};
use yapi_core::extractors::json::ValidateJson;
use yapi_core::{types::UserReg, Result, res::ResData};
use yapi_core::services::user_service;

pub fn router() -> Router {
    Router::new()
        .route("/user/reg", post(reg))
}

async fn reg(
    ValidateJson(req): ValidateJson<UserReg>
) -> Result<ResData> {
    user_service::reg(req).await;
    Ok(ResData::success(()))
}