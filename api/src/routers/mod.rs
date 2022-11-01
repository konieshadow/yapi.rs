use axum::Router;

mod user;
mod group;

pub fn routers() -> Router {
    let api_routers = user::router()
        .merge(group::router());
    Router::new().nest("/api", api_routers)
}