use api;
use dotenvy;
use serde::Serialize;
use serde_json::json;

fn main() {
        dotenvy::dotenv().expect(".env 文件读取失败");
        api::start();

    // let res_json = ResJson {
    //     code: 200,
    //     data: "123".into(),
    //     message: "success".into(),
    // };

    // let json_string = json!(res_json).to_string();

    // println!("{json_string}");
}

#[derive(Debug, Serialize)]
struct ResJson {
    code: i32,
    data: String,
    message: String,
}
