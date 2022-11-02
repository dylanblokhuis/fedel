use axum::{routing::get, Router};

mod admin;

// create admin router
pub fn admin_router() -> Router {
    Router::new().route(
        "/",
        get(admin::home::get_handler).post(admin::home::post_handler),
    )
}
