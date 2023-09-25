use crate::AppState;
use anyhow::Result;
use axum::{
    extract::{Json, Query, State},
    http::{header, HeaderName, HeaderValue},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use common::{error::CustErr, res::Res};
use core::posts;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    let res = test_service().await;

    println!("{:?}", state);

    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }
}

pub async fn create_first_post(
    state: State<AppState>,
    WithRejection(Json(param), _): WithRejection<Json<CreateFirstPostParam>, Res<()>>,
) -> impl IntoResponse {
    let res = posts::create_first_post(&state.db, param.title).await;

    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }
}

/**
 * 列表查询
 */
pub async fn query_list(
    WithRejection(Query(param), _): WithRejection<Query<QueryListParams>, Res<()>>,
) -> impl IntoResponse {
    println!("{:?}", param);
    "query_list"
}

/**
 * 按id 查询
 */
pub async fn query_by_id(
    WithRejection(Query(param), _): WithRejection<Query<ByIdParams>, Res<()>>,
) -> impl IntoResponse {
    "query_by_id"
}

/**
 * 创建post
 */
pub async fn create(
    WithRejection(Json(param), _): WithRejection<Json<CreateParams>, Res<()>>,
) -> impl IntoResponse {
    "create"
}

/**
 * 修改post
 */
pub async fn update(
    WithRejection(Json(param), _): WithRejection<Json<UpdateParams>, Res<()>>,
) -> impl IntoResponse {
    "update"
}

/**
 * 删除post
 */
pub async fn delete_by_id(
    WithRejection(Json(param), _): WithRejection<Json<ByIdParams>, Res<()>>,
) -> impl IntoResponse {
    "delete_by_id"
}

/// QueryListParams
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryListParams {
    page_number: Option<f64>,

    page_size: Option<f64>,

    title: Option<String>,
}

/// CreateParams
#[derive(Serialize, Deserialize)]
pub struct CreateParams {
    text: String,

    title: String,
}

/// UpdateParams
#[derive(Serialize, Deserialize)]
pub struct UpdateParams {
    id: f64,

    text: Option<String>,

    title: Option<String>,
}

/// ByIdParams
#[derive(Serialize, Deserialize)]
pub struct ByIdParams {
    id: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFirstPostParam {
    title: String,
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
