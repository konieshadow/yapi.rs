use axum::Extension;
use axum::{Router, routing::post};
use yapi_common::types::{UserReg, AuthUserInfo, UserLogin};
use yapi_core::extractors::auth::AuthUser;
use yapi_core::extractors::json::ValidateJson;
use yapi_core::{Result, res::ResData};
use yapi_core::services::user_service;

use crate::Context;

pub fn router() -> Router {
    Router::new()
        .route("/user/reg", post(reg))
        .route("/user/login", post(login))
}

async fn reg(
    ctx: Extension<Context>,
    ValidateJson(req): ValidateJson<UserReg>,
) -> Result<ResData<AuthUserInfo>> {
    let data = user_service::reg(&ctx.db, req).await?;

    let token = AuthUser::new(data.uid).to_jwt(&ctx.config.hmac_key);

    Ok(ResData::success(AuthUserInfo {
        user_info: data,
        token,
    }))
}

async fn login(
    ctx: Extension<Context>,
    ValidateJson(req): ValidateJson<UserLogin>,
) -> Result<ResData<AuthUserInfo>> {
    let data = user_service::login(&ctx.db, req).await?;

    let token = AuthUser::new(data.uid).to_jwt(&ctx.config.hmac_key);

    Ok(ResData::success(AuthUserInfo {
        user_info: data,
        token,
    }))
}