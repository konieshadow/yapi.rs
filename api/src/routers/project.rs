use axum::{routing::{post, get}, Router, Extension, extract::Query};
use yapi_common::types::{ProjectAdd, ProjectInfo, ProjectUp, UpdateResult, GetById, DeleteResult, ProjectDetail, ProjectList, List, ProjectItem};
use yapi_core::{Result, Context, extractors::{auth::AuthUser, json::ValidateJson}, res::ResData, services::project_service};

pub fn router() -> Router {
    Router::new()
        .route("/project/add", post(add))
        .route("/project/up", post(up))
        .route("/project/del", post(del))
        .route("/project/get", get(get_project))
        .route("/project/list", get(list))
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

async fn del(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<GetById>
) -> Result<ResData<DeleteResult>> {
    let data = project_service::del(&ctx.db, auth_user.id, req.id).await?;

    Ok(ResData::success(data))
}

async fn get_project(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<GetById>
) -> Result<ResData<ProjectDetail>> {
    let data = project_service::get(&ctx.db, auth_user.id, req.id).await?;

    Ok(ResData::success(data))
}

async fn list(
    ctx: Extension<Context>,
    auth_user: AuthUser,
    Query(req): Query<ProjectList>
) -> Result<ResData<List<ProjectItem>>> {
    let data = project_service::list(&ctx.db, auth_user.id, req).await?;

    Ok(ResData::success(data))
}