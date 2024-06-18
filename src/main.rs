use rusty_barn::{config, http::{self}};

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    http::serve(config::Config::new(), None).await;
}
