use crate::{
    api_calls::{create_login_session, info_dns_records, update_dns_records},
    configuration::Configuration,
};

pub struct ApiConnection {
    pub configuration: Configuration,
    pub session_id: Option<String>,
}

impl ApiConnection {
    pub async fn login(&mut self) {
        let login_response = create_login_session(&self.configuration.credentials).await;

        match login_response {
            Ok(response) => {
                self.session_id = Some(response.responsedata.apisessionid);
            }
            Err(_) => {
                self.session_id = None;
            }
        }
    }
    pub async fn logout(&self) {}

    pub async fn get_dns_records(&self, verbose: bool) {
        match &self.session_id {
            Some(session_id) => {
                verbose_print("Retrieving DNS records...", verbose);

                let cloned_domains = self.configuration.domains.clone();

                for (_, domain) in cloned_domains.into_iter() {
                    if domain.dns_records.is_none() {
                        continue;
                    }

                    verbose_print(
                        format!("Retrieving '{}'...", domain.domain_name).as_str(),
                        verbose,
                    );

                    let info_dns_records_response = info_dns_records(
                        session_id.to_string(),
                        domain.domain_name.to_string(),
                        &self.configuration.credentials,
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
            None => {
                verbose_print("Couldn't connect to netcup API", verbose);
            }
        }
    }

    pub async fn update_dns_records(&self, verbose: bool) {
        match &self.session_id {
            Some(session_id) => {
                verbose_print("Updating DNS records...", verbose);

                let cloned_domains = self.configuration.domains.clone();

                for (_, domain) in cloned_domains.into_iter() {
                    if domain.dns_records.is_none() {
                        continue;
                    }

                    verbose_print(
                        format!("Updating '{}'...", domain.domain_name).as_str(),
                        verbose,
                    );
                    let update_dns_records_response = update_dns_records(
                        session_id.to_string(),
                        domain.domain_name.to_string(),
                        &self.configuration.credentials,
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
            None => {
                verbose_print("Couldn't connect to netcup API", verbose);
            }
        };
    }
}

fn verbose_print(text: &str, verbose: bool) {
    if !verbose {
        return;
    }

    println!("{}", text);
}
