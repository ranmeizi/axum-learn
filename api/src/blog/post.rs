use crate::AppState;
use axum::extract::State;

pub async fn index_handler(state: State<AppState>) -> &'static str {
    println!("{:?}", state);
    "Hello Posts Index"
}
