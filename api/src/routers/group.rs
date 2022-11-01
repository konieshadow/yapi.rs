use axum::extract::Query;
use axum::routing::{post, get};
use axum::{Router, Extension};
use yapi_common::types::{GroupAdd, GroupWithMember, GetById};
use yapi_core::extractors::auth::AuthUser;
use yapi_core::services::group_service;
use yapi_core::{extractors::json::ValidateJson, res::ResData, Context};
use yapi_core::Result;

pub fn router() -> Router {
    Router::new()
        .route("/group/add", post(add))
        .route("/group/get", get(get_group))
}

async fn add(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<GroupAdd>
) -> Result<ResData<GroupWithMember>> {
    let data = group_service::add(&ctx.db, req, auth_user.user_id).await?;

    Ok(ResData::success(data))
}

async fn get_group(
    ctx: Extension<Context>,
    _: AuthUser,
    Query(req): Query<GetById>,
) -> Result<ResData<GroupWithMember>> {
    let data = group_service::get(&ctx.db, req.id).await?;

    Ok(ResData::success(data))
}