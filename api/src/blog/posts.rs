use crate::AppState;
use anyhow::Result;
use axum::{
    extract::{Json, Query, State},
    http::{header, HeaderName, HeaderValue},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use common::{error::CustErr, res::Res};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    let res = test_service().await;

    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A {
    a: String,
}

// from query
pub async fn from_query(
    WithRejection(Query(param), _): WithRejection<Query<A>, Res<()>>,
) -> impl IntoResponse {
    let data = format!("param from query a = {}", param.a);

    Res::success(data)
}

// from body
pub async fn from_body(
    WithRejection(Json(param), _): WithRejection<Json<A>, Res<()>>,
) -> impl IntoResponse {
    let data = format!("param from body a = {}", param.a);

    Res::success(data)
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
