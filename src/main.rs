use std::fs;

use api_calls::create_login_session;
use api_objects::CredentialsFile;

mod api_calls;
mod api_objects;

const CONFIG_FILE: &str = "example.toml";

#[tokio::main]
async fn main() {
    let config = read_config();
    let response = create_login_session(config).await;

    match response {
        Ok(res) => println!("{:?}", res.statuscode),
        Err(status_code) => println!("Status Code: {}", status_code),
    };
}

fn read_config() -> CredentialsFile {
    let file_content = fs::read_to_string(CONFIG_FILE).unwrap();

    toml::from_str(&file_content).unwrap()
}
