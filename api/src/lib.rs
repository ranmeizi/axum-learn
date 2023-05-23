use axum::{routing::get, Router};
use std::env;

mod blog;
mod routes;

#[tokio::main]
pub async fn start() {
    // 获取 port 值
    let port = env::var("PORT").expect("PORT is not set in file .env");

    let state = AppState {
        name : String::from("axum-learn")
    };

    // build our application with a single route
    let app = Router::new().nest("/", routes::compose()).with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct AppState {
    // dbconnection:
    name: String,
}

type StateRouter = Router<AppState>;
