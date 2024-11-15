use serde::{Deserialize, Serialize};

use crate::configuration::Credentials;

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub action: String,
    pub param: Credentials,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct LoginResponse {
    pub serverrequestid: String,
    pub clientrequestid: String,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: String,
    pub responsedata: ResponseData,
}

impl Default for LoginResponse {
    fn default() -> LoginResponse {
        LoginResponse {
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

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ResponseData {
    pub apisessionid: String,
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
pub struct InfoDomainResponse {
    pub serverrequestid: String,
    pub clientrequestid: String,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: String,
    pub responsedata: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateDnsRecordsRequest {
    pub domainname: String,
    pub customernumber: i32,
    pub apikey: String,
    pub apisessionid: String,
    pub clientrequestid: Option<String>,
    pub dnsrecordset: DnsRecordSet,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateDnsRecordsResponse {
    pub serverrequestid: String,
    pub clientrequestid: String,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: String,
    pub responsedata: String,
}

#[derive(Debug, Serialize)]
pub struct DnsRecordSet {
    pub dnsrecords: Vec<DnsRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsRecord {
    id: Option<i32>,
    hostname: String,
    dns_type: RecordType,
    priority: Option<String>,
    destination: String,
    delete: Option<bool>,
    state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(nonstandard_style)]
pub enum RecordType {
    A,
    AAAA,
    MX,
    TXT,
    CNAME,
    SRV,
    NS,
    DS,
    TLSA,
    CAA,
    SSHFP,
    SMIMEA,
    OPENPGPKEY,
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
