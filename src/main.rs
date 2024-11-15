use std::fs;

use api_calls::{create_login_session, get_domain_info, update_dns_records};
use configuration::Configuration;

mod api_calls;
mod api_objects;
mod configuration;

const CONFIG_FILE: &str = "example.toml";

#[tokio::main]
async fn main() {
    let config = read_config();
    let api_credentials = config.credentials.clone();
    let response = create_login_session(&config.credentials).await;

    match response {
        Ok(res) => {
            for (_, domain) in config.domains.into_iter() {
                /*let domain_repsonse = get_domain_info(
                    res.responsedata.apisessionid.to_owned(),
                    domain.domain_name.to_string(),
                    &api_credentials,
                )
                .await;*/

                if domain.dns_records.is_none() {
                    continue;
                }

                let update_dns_records_response = update_dns_records(
                    res.responsedata.apisessionid.to_owned(),
                    domain.domain_name.to_string(),
                    &api_credentials,
                    domain.dns_records.unwrap(),
                )
                .await;

                if update_dns_records_response.is_ok() {
                    let ok_update = update_dns_records_response.unwrap();
                    println!("{:?}", ok_update.statuscode);
                } else {
                    let error_code = update_dns_records_response.unwrap_err();
                    println!("{:?}", error_code);
                }
            }
        }
        Err(status_code) => println!("Status Code: {}", status_code),
    };
}

fn read_config() -> Configuration {
    let file_content = fs::read_to_string(CONFIG_FILE).unwrap();

    toml::from_str(&file_content).unwrap()
}
