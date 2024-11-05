use std::fs;

use api_calls::{create_login_session, get_domain_info};
use api_objects::CredentialsFile;

mod api_calls;
mod api_objects;

const CONFIG_FILE: &str = "example.toml";

#[tokio::main]
async fn main() {
    let config = read_config();
    let response = create_login_session(&config).await;

    match response {
        Ok(res) => {
            let domain_repsonse =
                get_domain_info(res.responsedata.apisessionid, "test.de".to_string(), config).await;

            if domain_repsonse.is_ok() {
                let ok_domain = domain_repsonse.unwrap();
                println!("{:?}", ok_domain.statuscode);
            } else {
                let error_code = domain_repsonse.unwrap_err();
                println!("{}", error_code);
            }
        }
        Err(status_code) => println!("Status Code: {}", status_code),
    };
}

fn read_config() -> CredentialsFile {
    let file_content = fs::read_to_string(CONFIG_FILE).unwrap();

    toml::from_str(&file_content).unwrap()
}
