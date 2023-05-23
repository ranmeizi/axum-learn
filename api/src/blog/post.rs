use crate::AppState;
use axum::{
    extract::State,
    http::{header, HeaderName, HeaderValue},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    let res_json = ResJson {
        code: 200,
        data: "123".into(),
        message: "success".into(),
    };

    res_json
}

// pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
//     let res_json = ResJson {
//         code: 200,
//         data: "123".into(),
//         message: "success".into(),
//     };

//     let json_string = json!(res_json).to_string();

//     // add header AppendHeaders

//     let header_part = AppendHeaders([(
//         header::CONTENT_TYPE,
//         HeaderValue::from_str("application/json").unwrap(),
//     )]);

//     // (part,intoresponse)
//     (header_part, json_string)
// }

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
