# Netcup DynDNS

DynDNS service for netcup nameservers

## Configuration

```toml
# example.toml
[credentials]
customernumber = 123456
apikey = "apikey"
apipassword = "apipassword"

[domains]
    [domains.first]
    domain_name = "first.domain"
    [domains.first.dns_records]
        [domains.first.dns_records.test]
        hostname = "test"
        dns_type = "A"
        destination = "1.2.3.4"

    [domains.second]
    domain_name = "second.domain"
```
