use axum::extract::Query;
use axum::{Extension};
use axum::{routing::{get, post}, Router};
use yapi_common::types::{UserReg, AuthUserInfo, UserLogin, UserInfo, PageList, Search, UserSearch, UserList};
use yapi_core::extractors::auth::{AuthUser, MaybeAuthUser};
use yapi_core::extractors::json::ValidateJson;
use yapi_core::{Result, res::ResData};
use yapi_core::services::user_service;

use crate::Context;

pub fn router() -> Router {
    Router::new()
        .route("/user/reg", post(reg))
        .route("/user/login", post(login))
        .route("/user/status", get(status))
        .route("/user/list", get(list))
        .route("/user/search", get(search))
}

async fn reg(
    ctx: Extension<Context>,
    ValidateJson(req): ValidateJson<UserReg>,
) -> Result<ResData<AuthUserInfo>> {
    let data = user_service::reg(&ctx.db, req).await?;

    let token = AuthUser::new(data.id).to_jwt(&ctx.config.hmac_key);

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

    let token = AuthUser::new(data.id).to_jwt(&ctx.config.hmac_key);

    Ok(ResData::success(AuthUserInfo {
        user_info: data,
        token,
    }))
}

async fn status(
    ctx: Extension<Context>,
    maybe_auth_user: MaybeAuthUser,
) -> Result<ResData<UserInfo>> {
    let data = user_service::status(&ctx.db, maybe_auth_user.user_id()).await?;

    Ok(ResData::success(data))
}

async fn list(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<UserList>,
) -> Result<ResData<PageList<UserInfo>>> {
    let data = user_service::list(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn search(
    ctx: Extension<Context>,
    _: AuthUser,
    Query(req): Query<Search>,
) -> Result<ResData<Vec<UserSearch>>> {
    let data = user_service::search(&ctx.db, req.q.as_str()).await?;

    Ok(ResData::success(data))
}