use axum::{routing::get, Router};
use crate::StateRouter;
mod post;

pub fn routes() -> StateRouter {
    Router::new().nest("/posts", post_api())
}

pub fn post_api() -> StateRouter {
    Router::new().route("/", get(post::index_handler))
}
