use crate::{blog, StateRouter};
use axum::Router;

pub fn compose() -> StateRouter {
    Router::new().nest("/blog", blog::routes())
}
