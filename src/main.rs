use std::{fs, path::PathBuf};

use api_calls::{create_login_session, get_domain_info, info_dns_records, update_dns_records};
use clap::{arg, command, Parser};
use configuration::Configuration;
use serde_json;

mod api_calls;
mod api_objects;
mod configuration;

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
            verbose_print("Parsing configuration file...", args.verbose);

            let config = read_config(args.config);
            verbose_print("Parsed configuration file", args.verbose);

            // currently precedence over info arg
            if args.test {
                verbose_print("Entering test branch", args.verbose);
                test_branch_command(config, args.verbose);
                return;
            }

            if args.info {
                verbose_print("Entering info branch", args.verbose);
                info_branch_command(config, args.verbose).await;
                return;
            }

            verbose_print("Entering default branch", args.verbose);
            default_branch_command(config, args.verbose).await;
        }
        Err(_) => {
            println!("Missing config argument");
        }
    }
}

async fn default_branch_command(config: Configuration, verbose: bool) {
    let api_credentials = config.credentials.clone();
    verbose_print("Retrieving login session to for netcup API...", verbose);
    let response = create_login_session(&config.credentials).await;
    verbose_print("Retrieved login session to for netcup API", verbose);

    match response {
        Ok(res) => {
            verbose_print("Updating DNS records...", verbose);
            for (_, domain) in config.domains.into_iter() {
                if domain.dns_records.is_none() {
                    continue;
                }

                verbose_print(
                    format!("Updating '{}'...", domain.domain_name).as_str(),
                    verbose,
                );
                let update_dns_records_response = update_dns_records(
                    res.responsedata.apisessionid.to_owned(),
                    domain.domain_name.to_string(),
                    &api_credentials,
                    domain.dns_records.unwrap(),
                )
                .await;

                if update_dns_records_response.is_ok() {
                    verbose_print(
                        format!("Updated '{}'", domain.domain_name).as_str(),
                        verbose,
                    );
                } else {
                    let error_code = update_dns_records_response.unwrap_err();
                    verbose_print(
                        format!("Couldn't update '{}'", domain.domain_name).as_str(),
                        verbose,
                    );
                    verbose_print(format!("Error: '{}'", error_code).as_str(), verbose);
                }
            }
            verbose_print("Updated DNS records", verbose);
        }
        Err(status_code) => {
            verbose_print("Couldn't connect to netcup API", verbose);
            verbose_print(format!("Error: '{}'", status_code).as_str(), verbose);
        }
    };
}

#[allow(dead_code)]
fn test_branch_command(config: Configuration, verbose: bool) {}

#[allow(dead_code)]
async fn info_branch_command(config: Configuration, verbose: bool) {
    let api_credentials = config.credentials.clone();
    verbose_print("Retrieving login session to for netcup API...", verbose);
    let response = create_login_session(&config.credentials).await;
    verbose_print("Retrieved login session to for netcup API", verbose);

    match response {
        Ok(res) => {
            verbose_print("Retrieving DNS records...", verbose);
            for (_, domain) in config.domains.into_iter() {
                if domain.dns_records.is_none() {
                    continue;
                }

                verbose_print(
                    format!("Retrieving '{}'...", domain.domain_name).as_str(),
                    verbose,
                );
                let info_dns_records_response = info_dns_records(
                    res.responsedata.apisessionid.to_owned(),
                    domain.domain_name.to_string(),
                    &api_credentials,
                )
                .await;

                if info_dns_records_response.is_ok() {
                    verbose_print(
                        format!("Retrieved '{}'", domain.domain_name).as_str(),
                        verbose,
                    );

                    let _json_dns_records = serde_json::to_string(
                        &info_dns_records_response.unwrap().responsedata.unwrap(),
                    );

                    // TODO: How to display data?
                } else {
                    let error_code = info_dns_records_response.unwrap_err();
                    verbose_print(
                        format!("Couldn't retrieve '{}'", domain.domain_name).as_str(),
                        verbose,
                    );
                    verbose_print(format!("Error: '{}'", error_code).as_str(), verbose);
                }
            }
            verbose_print("Retrieved DNS records", verbose);
        }
        Err(status_code) => {
            verbose_print("Couldn't connect to netcup API", verbose);
            verbose_print(format!("Error: '{}'", status_code).as_str(), verbose);
        }
    };
}

fn read_config(config_file: PathBuf) -> Configuration {
    let file_content = fs::read_to_string(config_file).unwrap();

    toml::from_str(&file_content).unwrap()
}

fn verbose_print(text: &str, verbose: bool) {
    if !verbose {
        return;
    }

    println!("{}", text);
}
