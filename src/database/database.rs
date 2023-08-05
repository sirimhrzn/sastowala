use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

#[derive(Clone)]
pub struct DBstate {
    pub db: MySqlPool,
}

pub async fn db_connection() -> DBstate {
    let _db_url =
        env::var("DATABASE_URL").expect("Database Url not found or invalid in environment");
    let host = env::var("DB_HOST").expect("Db host not found or invalid in environment");
    let username =
        env::var("DB_USERNAME").expect("Database UserName not found or invalid in environment");
    let password =
        env::var("DB_PASSWORD").expect("Database Password not found or invalid in environment");
    let db_name = env::var("DB_NAME").expect("Database Name not found or invalid in environment");
    let db_port = env::var("DB_PORT").expect("Database Port not found or invalid in environment");

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        username, password, host, db_port, db_name
    );
    let pool = MySqlPoolOptions::new().connect(&url).await.unwrap();
    DBstate { db: pool }
}
