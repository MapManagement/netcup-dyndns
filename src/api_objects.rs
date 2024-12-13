use serde::{Deserialize, Serialize};

use crate::configuration::Credentials;

// =============================

#[derive(Debug, Serialize)]
pub struct LoginRequestFrame {
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
    pub responsedata: LoginResponseData,
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
            responsedata: LoginResponseData {
                apisessionid: "default".to_string(),
            },
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct LoginResponseData {
    pub apisessionid: String,
}

// =============================

#[derive(Debug, Serialize)]
pub struct InfoDomainRequestFrame {
    pub action: String,
    pub param: InfoDomainRequest,
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

// =============================
//
#[derive(Debug, Serialize)]
pub struct InfoDnsRecordsFrame {
    pub action: String,
    pub param: InfoDnsRecordsRequest,
}

#[derive(Debug, Serialize)]
pub struct InfoDnsRecordsRequest {
    pub domainname: String,
    pub customernumber: i32,
    pub apikey: String,
    pub apisessionid: String,
    pub clientrequestid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct InfoDnsRecordsResponse {
    pub serverrequestid: String,
    pub clientrequestid: String,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: String,
    pub responsedata: Option<DnsRecordsResponseData>,
}

// =============================

#[derive(Debug, Serialize)]
pub struct UpdateDnsRecordsRequestFrame {
    pub action: String,
    pub param: UpdateDnsRecordsRequest,
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
    pub clientrequestid: Option<String>,
    pub action: String,
    pub status: String,
    pub statuscode: i32,
    pub shortmessage: String,
    pub longmessage: Option<String>,
    pub responsedata: Option<DnsRecordsResponseData>,
}

// =============================

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct DnsRecordsResponseData {
    pub dnsrecords: Vec<DnsRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecordSet {
    pub dnsrecords: Vec<DnsRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsRecord {
    id: Option<String>,
    hostname: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    dns_type: RecordType,
    priority: Option<String>,
    destination: String,
    deleterecord: Option<bool>,
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
