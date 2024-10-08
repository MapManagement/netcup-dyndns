use std::fs;

use api_calls::{create_login_session, ApiLogin};

mod api_calls;

const CONFIG_FILE: &str = "example.toml";

#[tokio::main]
async fn main() {
    let config = read_config();
    let response = create_login_session(config).await.unwrap();
    println!(
        "Status: {}, Message: {}",
        response.status, response.shortmessage
    );
}

fn read_config() -> ApiLogin {
    let file_content = fs::read_to_string(CONFIG_FILE).unwrap();

    toml::from_str(&file_content).unwrap()
}
