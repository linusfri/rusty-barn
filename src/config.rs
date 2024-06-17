#[derive(Debug)]
pub struct Config {
    pub connection_string: Option<String>,
    pub address_and_port: String
    // For authentication
    //pub jwt_secret: String
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let address_and_port = std::env::var("ADDRESS_AND_PORT").expect("ADDRESS_AND_PORT must be set.");

        Config { connection_string: None, address_and_port }
    }
}