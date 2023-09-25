use api;
use dotenvy;

fn main() {
    dotenvy::dotenv().expect(".env 文件读取失败");
    api::start();
}
