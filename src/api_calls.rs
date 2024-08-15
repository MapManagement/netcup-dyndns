struct ApiAuth {
    domain: String,
    customernumber: u32,
    api_key: String,
    session_id: String
}

enum RecordType {
    A,
    AAAA,
    MX,
    CNAME,
    TXT,
    NS,
    SOA,
    SRV
}

struct DnsRecord {
    id: u32,
    hostname: String,
    dns_type: RecordType,
    priority: String,
    destination: String,
    delete: bool,
    state: String
}

struct DnsZone {
    domain: String,
    ttl: u32,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    enable_dnsssec: bool
}

async fn update_dns_records(auth: ApiAuth) {
}

async fn update_dns_zone(auth: ApiAuth) {
}

async fn update_domain(auth: ApiAuth) {
}

async fn update_handle(auth: ApiAuth) {
}
