use crate::api_objects::*;
use crate::configuration::*;

use std::collections::HashMap;

const API_URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";
const SUCCESS_STATUS_CODE: i32 = 2000;

const LOGIN_ACTION: &str = "login";
const LOGOUT_ACTION: &str = "logout";
const INFO_DOMAIN_ACTION: &str = "infoDomain";
const UPDATE_DNS_RECORDS_ACTION: &str = "updateDnsRecords";
const INFO_DNS_RECORDS_ACTION: &str = "infoDnsRecords";

pub async fn create_login_session(api_login: &Credentials) -> Result<LoginResponse, i32> {
    let login_request = LoginRequestFrame {
        action: LOGIN_ACTION.to_string(),
        param: Credentials {
            customernumber: api_login.customernumber,
            apikey: api_login.apikey.clone(),
            apipassword: api_login.apipassword.clone(),
        },
    };

    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .json(&login_request)
        .send()
        .await
        .expect("Couldn't log into Netcup API.")
        .json::<LoginResponse>()
        .await
        .expect("Couldn't parse response of Netcup API.");

    match response.statuscode {
        SUCCESS_STATUS_CODE => Ok(response),
        _ => Err(response.statuscode),
    }
}

pub async fn destroy_login_session(api_login: &Credentials, session_id: String) {
    let logout_request = LogoutRequestFrame {
        action: LOGOUT_ACTION.to_string(),
        param: LogoutRequest {
            customernumber: api_login.customernumber,
            apikey: api_login.apikey.clone(),
            apisessionid: session_id,
            clientrequestid: None,
        },
    };

    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .json(&logout_request)
        .send()
        .await
        .expect("Couldn't logout of Netcup API.")
        .json::<LoginResponse>()
        .await
        .expect("Couldn't parse response of Netcup API.");
}

// ==== read information ====

/// Only available for domain resellers
pub async fn get_domain_info(
    session_id: String,
    domain: String,
    credentials: &Credentials,
) -> Result<InfoDomainResponse, i32> {
    let domain_info_request = InfoDomainRequest {
        domainname: domain,
        customernumber: credentials.customernumber,
        apikey: credentials.apikey.to_owned(),
        apisessionid: session_id,
        clientrequestid: None,
        registryinformationflag: None,
    };

    let request_frame = InfoDomainRequestFrame {
        action: INFO_DOMAIN_ACTION.to_string(),
        param: domain_info_request,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .json(&request_frame)
        .send()
        .await
        .expect("Couldn't retrieve data from Netcup API.")
        .json::<InfoDomainResponse>()
        .await
        .expect("Couldn't parse response of Netcup API.");

    match response.statuscode {
        SUCCESS_STATUS_CODE => Ok(response),
        _ => {
            println!("{:?}", response.longmessage);
            Err(response.statuscode)
        }
    }
}

pub async fn info_dns_records(
    session_id: String,
    domain: String,
    credentials: &Credentials,
) -> Result<InfoDnsRecordsResponse, i32> {
    let info_dns_records_request = InfoDnsRecordsRequest {
        domainname: domain,
        customernumber: credentials.customernumber,
        apikey: credentials.apikey.to_owned(),
        apisessionid: session_id,
        clientrequestid: None,
    };

    let request_frame = InfoDnsRecordsFrame {
        action: INFO_DNS_RECORDS_ACTION.to_string(),
        param: info_dns_records_request,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .json(&request_frame)
        .send()
        .await
        .expect("Couldn't retrieve data from Netcup API.")
        .json::<InfoDnsRecordsResponse>()
        .await
        .expect("Couldn't parse response of Netcup API.");

    match response.statuscode {
        SUCCESS_STATUS_CODE => Ok(response),
        _ => {
            println!("{:?}", response.longmessage);
            Err(response.statuscode)
        }
    }
}

// ==== update information ====

pub async fn update_dns_records(
    session_id: String,
    domain: String,
    credentials: &Credentials,
    dns_records: HashMap<String, DnsRecord>,
) -> Result<UpdateDnsRecordsResponse, i32> {
    let vec_dns_records: Vec<DnsRecord> = dns_records.into_values().collect();

    let update_dns_records_request = UpdateDnsRecordsRequest {
        domainname: domain,
        customernumber: credentials.customernumber,
        apikey: credentials.apikey.to_owned(),
        apisessionid: session_id,
        clientrequestid: None,
        dnsrecordset: DnsRecordSet {
            dnsrecords: vec_dns_records,
        },
    };

    let request_frame = UpdateDnsRecordsRequestFrame {
        action: UPDATE_DNS_RECORDS_ACTION.to_string(),
        param: update_dns_records_request,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .json(&request_frame)
        .send()
        .await
        .expect("Couldn't retrieve data from Netcup API.")
        .json::<UpdateDnsRecordsResponse>()
        .await
        .expect("Couldn't parse response of Netcup API.");

    match response.statuscode {
        SUCCESS_STATUS_CODE => Ok(response),
        _ => {
            println!("{:?}", response.longmessage);
            Err(response.statuscode)
        }
    }
}

pub async fn update_dns_zone() {}

pub async fn update_domain() {}

pub async fn update_handle() {}
