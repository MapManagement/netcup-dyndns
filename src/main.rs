use std::{fs, path::PathBuf};

use api_calls::{create_login_session, get_domain_info, update_dns_records};
use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use configuration::Configuration;

mod api_calls;
mod api_objects;
mod configuration;

const CONFIG_ARG: &str = "config";
const TEST_ARG: &str = "test";
const VERBOSE_ARG: &str = "verbose";
const INFO_ARG: &str = "info";

#[tokio::main]
async fn main() {
    let command_matches = command!()
        .arg(Arg::new(CONFIG_ARG)
                .short('c')
                .long(CONFIG_ARG)
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(PathBuf))
                .help("Specify the file to read the domain configuration from")
        )
        .arg(
            Arg::new(TEST_ARG)
                .short('t')
                .long(TEST_ARG)
                .required(false)
                .num_args(0)
                .help("Verifies whether the specified configuration file is formatted correctly"),
        )
        .arg(
            Arg::new(VERBOSE_ARG)
                .short('v')
                .long(VERBOSE_ARG)
                .required(false)
                .num_args(0)
                .help("Prints more detailed information about API calls and errors."),
        )
        .arg(
            Arg::new(INFO_ARG)
                .short('i')
                .long(INFO_ARG)
                .required(false)
                .num_args(0)
                .help("Retrieves information about the specified domains and prints it in a readable format."),
        ).get_matches();

    if let Some(config_file) = command_matches.get_one::<PathBuf>(CONFIG_ARG) {
        let config = read_config(config_file.to_path_buf());
        let verbose_flag = command_matches.contains_id(VERBOSE_ARG);

        // currently precedence over info arg
        if command_matches.contains_id(TEST_ARG) {
            test_branch_command(config, verbose_flag);
            return;
        }

        if command_matches.contains_id(INFO_ARG) {
            info_branch_command(config, verbose_flag).await;
            return;
        }

        default_branch_command(config, verbose_flag).await;
    }

    println!("Missing config argument");
}

async fn default_branch_command(config: Configuration, verbose: bool) {
    let api_credentials = config.credentials.clone();
    let response = create_login_session(&config.credentials).await;

    match response {
        Ok(res) => {
            for (_, domain) in config.domains.into_iter() {
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

#[allow(dead_code)]
fn test_branch_command(config: Configuration, verbose: bool) {}

#[allow(dead_code)]
async fn info_branch_command(config: Configuration, verbose: bool) {}

fn read_config(config_file: PathBuf) -> Configuration {
    let file_content = fs::read_to_string(config_file).unwrap();

    toml::from_str(&file_content).unwrap()
}
