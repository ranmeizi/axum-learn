use crate::error::CustErr;
use anyhow::Error;
use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;

/**
 * {
 *     code:200,
 *     data:{...}  T,
 *     message:"message"
 * }
 */
#[derive(Debug, Serialize)]
pub struct Res<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T> Res<T> {
    /**
     * Res{
     *     code:200,
     *     data:data,
     *     message:"success"
     * }
     */
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: String::from("success"),
        }
    }

    /**
     * Res{
     *     code:400/xxx,
     *     data:None,
     *     message:"error message"
     * }
     */
    pub fn error(e: Error) -> Self {
        let code = if e.downcast_ref::<CustErr>().is_some() {
            // 预期的错误返回 code 码
            match e.downcast_ref::<CustErr>() {
                Some(CustErr::AppRuleError(_)) => 400,
                _ => 400,
            }
        } else {
            // 非预期的错误返回 500
            500
        };

        Self {
            code: code,
            data: None,
            message: e.to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> axum::response::Response {
        let val = json!(self);

        Json(val).into_response()
    }
}

impl From<JsonRejection> for Res<()> {
    fn from(value: JsonRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            data: None,
            message: value.body_text(),
        }
    }
}

impl From<QueryRejection> for Res<()> {
    fn from(value: QueryRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            data: None,
            message: value.body_text(),
        }
    }
}

