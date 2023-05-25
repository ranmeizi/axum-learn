use crate::AppState;
use anyhow::Result;
use axum::{
    extract::State,
    http::{header, HeaderName, HeaderValue},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use common::{error::CustErr, res::Res};
use serde::Serialize;
use serde_json::json;

pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    let res = test_service().await;

    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }
}

pub async fn test_service() -> Result<String> {
    Err(CustErr::AppRuleError("对不起，触犯天条了".into()).into())
}

#[derive(Debug, Serialize)]
struct ResJson {
    code: i32,
    data: String,
    message: String,
}

impl IntoResponse for ResJson {
    fn into_response(self) -> axum::response::Response {
        let val = json!(self);

        Json(val).into_response()
    }
}
