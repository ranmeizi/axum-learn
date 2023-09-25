use crate::StateRouter;
use axum::{
    routing::{get, post},
    Router,
};
mod posts;

pub fn routes() -> StateRouter {
    Router::new().nest("/posts", post_api())
}

pub fn post_api() -> StateRouter {
    Router::new()
        .route("/", get(posts::index_handler))
        .route("/query", get(posts::from_query))
        .route("/body", post(posts::from_body))
        .route("/create_test", post(posts::create_first_post))
        .route("/list", get(posts::query_list))
        .route("/create", post(posts::create))
        .route("/update", post(posts::update))
        .route("/findById", get(posts::query_by_id))
        .route("/deleteById", post(posts::delete_by_id))
}
