use std::{fs, path::PathBuf};

use api_calls::{create_login_session, get_domain_info, update_dns_records};
use clap::{arg, command, value_parser, Arg, ArgAction, Command, Parser};
use configuration::Configuration;

mod api_calls;
mod api_objects;
mod configuration;

const CONFIG_ARG: &str = "config";
const TEST_ARG: &str = "test";
const VERBOSE_ARG: &str = "verbose";
const INFO_ARG: &str = "info";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specify the file to read the domain configuration from
    #[arg(short, long)]
    config: PathBuf,

    /// Verifies whether the specified configuration file is formatted correctly
    #[arg(short, long, required = false, num_args = 0)]
    test: bool,

    /// Prints more detailed information about API calls and errors
    #[arg(short, long, required = false, num_args = 0)]
    verbose: bool,

    /// Retrieves information about the specified domains and prints it in a readable format
    #[arg(short, long, required = false, num_args = 0)]
    info: bool,
}

#[tokio::main]
async fn main() {
    let tried_args = Args::try_parse();

    match tried_args {
        Ok(args) => {
            let config = read_config(args.config);

            // currently precedence over info arg
            if args.test {
                test_branch_command(config, args.verbose);
                return;
            }

            if args.info {
                info_branch_command(config, args.verbose).await;
                return;
            }

            default_branch_command(config, args.verbose).await;
        }
        Err(error) => {
            println!("Missing config argument");
        }
    }
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
