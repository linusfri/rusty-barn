use rusty_barn::{config, http::{self}};

#[tokio::main]
async fn main() {
    http::serve(config::Config::new(), None).await;
}
