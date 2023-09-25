use axum::{routing::get, Router};
use std::env;
use sea_orm::DatabaseConnection;
use common::db_conn;

mod blog;
mod routes;

#[tokio::main]
pub async fn start() {
    // 获取 port 值
    let port = env::var("PORT").expect("PORT is not set in file .env");

    let state = AppState {
        db : db_conn::get_db_conn().await
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
    db: DatabaseConnection,
}

type StateRouter = Router<AppState>;
