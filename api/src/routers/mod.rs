use axum::Router;

mod user;
mod group;
mod project;
mod interface;

pub fn routers() -> Router {
    let api_routers = user::router()
        .merge(group::router())
        .merge(project::router())
        .merge(interface::router());
    Router::new().nest("/api", api_routers)
}