use rusty_barn::{config, http::{self}};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    http::serve(config::Config::new(), None).await;
}
