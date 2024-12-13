use std::{fs, path::PathBuf};

use api_calls::{create_login_session, get_domain_info, info_dns_records, update_dns_records};
use clap::{arg, command, Parser};
use configuration::Configuration;
use domain_utils::ApiConnection;
use serde_json;

mod api_calls;
mod api_objects;
mod configuration;
mod domain_utils;

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
    let mut api_connection = ApiConnection {
        configuration: config,
        session_id: None,
    };
    verbose_print("Retrieving login session to for netcup API...", verbose);

    api_connection.login().await;

    verbose_print("Retrieved login session to for netcup API", verbose);

    api_connection.update_dns_records(verbose).await;
}

#[allow(dead_code)]
fn test_branch_command(config: Configuration, verbose: bool) {}

#[allow(dead_code)]
async fn info_branch_command(config: Configuration, verbose: bool) {
    let mut api_connection = ApiConnection {
        configuration: config,
        session_id: None,
    };
    verbose_print("Retrieving login session to for netcup API...", verbose);

    api_connection.login().await;

    verbose_print("Retrieved login session to for netcup API", verbose);

    api_connection.get_dns_records(verbose).await;
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
