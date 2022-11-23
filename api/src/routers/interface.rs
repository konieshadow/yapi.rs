use axum::{Router, Extension, routing::post, extract::Query, Json};
use yapi_common::types::{InterfaceCatAdd, InterfaceCat, InterfaceCatUp, UpdateResult, GetById, DeleteResult, IndexItem, InterfaceAdd, InterfaceInfo};
use yapi_core::{Context, extractors::{auth::AuthUser, json::ValidateJson}, res::ResData, Result, services::{interface_cat_service, interface_service}};

pub fn router() -> Router {
    Router::new()
        .route("/interface/add_cat", post(add_interface_cat))
        .route("/interface/up_cat", post(up_interface_cat))
        .route("/interface/del_cat", post(delete_interface_cat))
        .route("/interface/up_cat_index", post(up_interface_cat_index))
        .route("/interface/add", post(add_interface))
}

async fn add_interface_cat(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<InterfaceCatAdd>
) -> Result<ResData<InterfaceCat>> {
    let data = interface_cat_service::add_interface_cat(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn up_interface_cat(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<InterfaceCatUp>
) -> Result<ResData<UpdateResult>> {
    let data = interface_cat_service::up_interface_cat(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn delete_interface_cat(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<GetById>
) -> Result<ResData<DeleteResult>> {
    let data = interface_cat_service::delete_interface_cat(&ctx.db, auth_user.id, req.id).await?;

    Ok(ResData::success(data))
}

async fn up_interface_cat_index(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Json(req): Json<Vec<IndexItem>>
) -> Result<ResData<UpdateResult>> {
    let data = interface_cat_service::up_interface_cat_index(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn add_interface(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<InterfaceAdd>
) -> Result<ResData<InterfaceInfo>> {
    let data = interface_service::add(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}