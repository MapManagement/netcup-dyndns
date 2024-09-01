use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const LOGIN_URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";

#[derive(Deserialize)]
pub struct ApiLogin {
    customer_number: u32,
    api_key: String,
    api_password: String,
}

pub struct ApiAuth {
    domain: String,
    customernumber: u32,
    api_key: String,
    session_id: String,
}

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
pub struct ResponseMessage {
    serverrequestedid: String,
    clientrequestedid: String,
    action: String,
    status: String,
    statuscode: u16,
    shortmessage: String,
    longmessage: String,
    responsedate: String,
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

pub async fn create_login_session(api_login: ApiLogin) -> Result<ResponseMessage, reqwest::Error> {
    let mut json_map = HashMap::new();
    json_map.insert("customernumber", api_login.customer_number.to_string());
    json_map.insert("apikey", api_login.api_key.to_string());
    json_map.insert("apipassword", api_login.api_password.to_string());

    let client = reqwest::Client::new();
    let repsonse = client.post(LOGIN_URL).json(&json_map).send().await;

    match repsonse {
        Ok(res) => res.json::<ResponseMessage>().await,
        Err(error) => Err(error),
    }
}

pub async fn update_dns_records(auth: ApiAuth) {}

pub async fn update_dns_zone(auth: ApiAuth) {}

pub async fn update_domain(auth: ApiAuth) {}

pub async fn update_handle(auth: ApiAuth) {}
