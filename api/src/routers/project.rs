use axum::{routing::post, Router, Extension};
use yapi_common::types::{ProjectAdd, ProjectInfo, ProjectUp, UpdateResult};
use yapi_core::{Result, Context, extractors::{auth::AuthUser, json::ValidateJson}, res::ResData, services::project_service};

pub fn router() -> Router {
    Router::new()
        .route("/project/add", post(add))
        .route("/project/up", post(up))
}

async fn add(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<ProjectAdd>
) -> Result<ResData<ProjectInfo>> {
    let data = project_service::add(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}

async fn up(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<ProjectUp>
) -> Result<ResData<UpdateResult>> {
    let data = project_service::up(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}