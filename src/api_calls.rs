use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const LOGIN_URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialsFile {
    pub customernumber: i32,
    pub apikey: String,
    pub apipassword: String,
}

#[derive(Debug, Serialize)]
pub struct ApiRequest {
    pub action: String,
    pub param: CredentialsFile,
}

pub struct ApiAuth {
    domain: String,
    customernumber: u32,
    api_key: String,
    session_id: String,
}

#[allow(nonstandard_style)]
pub enum RecordType {
    A,
    AAAA,
    MX,
    CNAME,
    TXT,
    NS,
    SOA,
    SRV,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ResponseData {
    pub apisessionid: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ResponseMessage {
    pub serverrequestid: String,
    pub clientrequestid: String,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: String,
    pub responsedata: ResponseData,
}

impl Default for ResponseMessage {
    fn default() -> ResponseMessage {
        ResponseMessage {
            serverrequestid: "default".to_string(),
            clientrequestid: "default".to_string(),
            action: "default".to_string(),
            status: "default".to_string(),
            statuscode: 0,
            shortmessage: "default".to_string(),
            longmessage: "default".to_string(),
            responsedata: ResponseData {
                apisessionid: "default".to_string(),
            },
        }
    }
}

pub struct DnsRecord {
    id: u32,
    hostname: String,
    dns_type: RecordType,
    priority: String,
    destination: String,
    delete: bool,
    state: String,
}

pub struct DnsZone {
    domain: String,
    ttl: u32,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    enable_dnsssec: bool,
}

pub async fn create_login_session(
    api_login: CredentialsFile,
) -> Result<ResponseMessage, reqwest::Error> {
    let login_request = ApiRequest {
        action: "login".to_string(),
        param: api_login,
    };

    let client = reqwest::Client::new();
    let response = client.post(LOGIN_URL).json(&login_request).send().await;

    match response {
        Ok(res) => res.json::<ResponseMessage>().await,
        Err(error) => Err(error),
    }
}

pub async fn update_dns_records(auth: ApiAuth) {}

pub async fn update_dns_zone(auth: ApiAuth) {}

pub async fn update_domain(auth: ApiAuth) {}

pub async fn update_handle(auth: ApiAuth) {}
