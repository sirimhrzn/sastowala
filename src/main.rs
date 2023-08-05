use dotenv::dotenv;
use sastowala::server;

#[tokio::main]
async fn main() {
    dotenv().ok();
    server().await;
}
