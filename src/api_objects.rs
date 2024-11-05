use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct InfoDomainRequest {
    pub domainname: String,
    pub customernumber: i32,
    pub apikey: String,
    pub apisessionid: String,
    pub clientrequestid: Option<String>,
    pub registryinformationflag: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ResponseInfoDomain {
    pub serverrequestid: String,
    pub clientrequestid: String,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: String,
    pub responsedata: String,
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
