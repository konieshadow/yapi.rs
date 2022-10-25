use axum::Extension;
use axum::{Router, routing::post};
use yapi_common::types::{UserReg, UserInfo};
use yapi_core::extractors::json::ValidateJson;
use yapi_core::{Result, res::ResData};
use yapi_core::services::user_service;

use crate::Context;

pub fn router() -> Router {
    Router::new()
        .route("/user/reg", post(reg))
}

async fn reg(
    ctx: Extension<Context>,
    ValidateJson(req): ValidateJson<UserReg>
) -> Result<ResData<UserInfo>> {
    let data = user_service::reg(&ctx.db, req).await?;
    Ok(ResData::success(data))
}