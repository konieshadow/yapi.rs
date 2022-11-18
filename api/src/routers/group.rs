use axum::extract::Query;
use axum::routing::{post, get};
use axum::{Router, Extension};
use yapi_common::types::{GroupAdd, GroupWithMember, GetById, GroupInfo, GroupUp, UpdateResult, DeleteResult};
use yapi_core::extractors::auth::AuthUser;
use yapi_core::services::group_service;
use yapi_core::{extractors::json::ValidateJson, res::ResData, Context};
use yapi_core::Result;

pub fn router() -> Router {
    Router::new()
        .route("/group/add", post(add))
        .route("/group/up", post(up))
        .route("/group/del", post(del))
        .route("/group/get", get(get_group))
        .route("/group/list", get(list_group))
}

async fn add(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<GroupAdd>
) -> Result<ResData<GroupWithMember>> {
    let data = group_service::add(&ctx.db, req, auth_user.user_id).await?;

    Ok(ResData::success(data))
}

async fn up(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<GroupUp>
) -> Result<ResData<UpdateResult>> {
    let data = group_service::up(&ctx.db, req, auth_user.user_id).await?;

    Ok(ResData::success(data))
}

async fn del(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<GetById>,
) -> Result<ResData<DeleteResult>> {
    let data = group_service::del(&ctx.db, auth_user.user_id, req.id).await?;

    Ok(ResData::success(data))
}

async fn get_group(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<GetById>,
) -> Result<ResData<GroupInfo>> {
    let data = group_service::get(&ctx.db, auth_user.user_id, req.id).await?;

    Ok(ResData::success(data))
}

async fn list_group(
    ctx: Extension<Context>,
    auth_user: AuthUser,
) -> Result<ResData<Vec<GroupInfo>>> {
    let data = group_service::list(&ctx.db, auth_user.user_id).await?;

    Ok(ResData::success(data))
}