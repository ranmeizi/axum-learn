use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn get_db_conn() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in file .env");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await.expect("数据库链接失败");

    db
}
