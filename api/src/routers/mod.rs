use axum::Router;

mod user;

pub fn routers() -> Router {
    let api_routers = user::router();
    Router::new().nest("/api", api_routers)
}