use axum::{Router, Extension, routing::post, extract::Query};
use yapi_common::types::{AddInterfaceCat, InterfaceCat, UpInterfaceCat, UpdateResult, GetById, DeleteResult};
use yapi_core::{Context, extractors::{auth::AuthUser, json::ValidateJson}, res::ResData, Result, services::interface_service};

pub fn router() -> Router {
    Router::new()
        .route("/interface/add_cat", post(add_interface_cat))
        .route("/interface/up_cat", post(up_interface_cat))
        .route("/interface/del_cat", post(delete_interface_cat))
}

async fn add_interface_cat(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<AddInterfaceCat>
) -> Result<ResData<InterfaceCat>> {
    let data = interface_service::add_interface_cat(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn up_interface_cat(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<UpInterfaceCat>
) -> Result<ResData<UpdateResult>> {
    let data = interface_service::up_interface_cat(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn delete_interface_cat(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<GetById>
) -> Result<ResData<DeleteResult>> {
    let data = interface_service::delete_interface_cat(&ctx.db, auth_user.id, req.id).await?;

    Ok(ResData::success(data))
}