use axum::{routing::{get,post}, Router};
use crate::StateRouter;
mod posts;

pub fn routes() -> StateRouter {
    Router::new().nest("/posts", post_api())
}

pub fn post_api() -> StateRouter {
    Router::new().route("/", get(posts::index_handler))
    .route("/query", get(posts::from_query))
    .route("/body", post(posts::from_body))
}
