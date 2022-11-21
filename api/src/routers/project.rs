use axum::{routing::post, Router, Extension};
use yapi_common::types::{ProjectAdd, ProjectInfo};
use yapi_core::{Result, Context, extractors::{auth::AuthUser, json::ValidateJson}, res::ResData, services::{group_service, project_service}};

pub fn router() -> Router {
    Router::new()
        .route("/project/add", post(add))
}

async fn add(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    ValidateJson(req): ValidateJson<ProjectAdd>
) -> Result<ResData<ProjectInfo>> {
    let data = project_service::add(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}