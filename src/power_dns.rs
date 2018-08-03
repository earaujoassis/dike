#[allow(unused)]
use rocket_contrib::{Json, Value};

/*
 * This method is used to do the basic query. It can omit auth, but if PowerDNS is using DNSSEC this can
 * lead into trouble.
 * Query example:
    {
        "method": "lookup",
        "parameters": {"
            qtype": "ANY",
            "qname": "www.example.com.",
            "remote": "192.0.2.24",
            "local": "192.0.2.1",
            "real-remote": "192.0.2.24",
            "zone-id":-1
        }
    }
 * Response example:
    {
        "result": [{"qtype": "A", "qname": "www.example.com", "content": "203.0.113.2", "ttl": 60}]
    }
 */
#[get("/lookup/<domain>/<qtype>")]
pub fn dns_lookup(domain: String, qtype: String) -> &'static str {
    "Mockup"
}

/*
 * Lists all records for the zonename. If PowerDNS is running DNSSEC, you should take care of setting
 * auth to appropriate value, otherwise things can go wrong.
 * Query example: {"method":"list", "parameters":{"zonename":"example.com.","domain_id":-1}}
 * Response example:
    {
        "result":[
            {
                "qtype": "SOA",
                "qname": "example.com",
                "content": "dns1.icann.org. hostmaster.icann.org. 2012081600 7200 3600 1209600 3600",
                "ttl": 3600
            },
            {"qtype": "NS", "qname": "example.com", "content": "ns1.example.com", "ttl": 60},
            {"qtype": "MX", "qname": "example.com", "content": "10 mx1.example.com.", "ttl": 60},
            {"qtype": "A", "qname": "www.example.com", "content": "203.0.113.2", "ttl": 60},
            {"qtype": "A", "qname": "ns1.example.com", "content": "192.0.2.2", "ttl": 60},
            {"qtype": "A", "qname": "mx1.example.com", "content": "192.0.2.3", "ttl": 60}
        ]
    }
 */
#[get("/list/<domain_id>/<domain>")]
pub fn dns_list(domain_id: i32, domain: String) -> &'static str {
    "Mockup"
}

/*
 * Lists all records for the zonename. If PowerDNS is running DNSSEC, you should take care of setting
 * auth to appropriate value, otherwise things can go wrong.
 * Query example:
    {
        "method": "adddomainkey",
        "parameters": {
            "key": {
                "id": 1,
                "flags": 256,
                "active": true,
                "content": "<omitted>"
            }
        }
    }
 * Response example: {"result": true}
 */
#[get("/adddomainkey/<domain>/<domain_id>")]
pub fn dns_add_domain_key(domain: String, domain_id: i32) -> &'static str {
    "Mockup"
}

/*
 * Retrieves any keys of kind. The id, flags are unsigned integers, and active is boolean. Content must
 * be valid key record in format that PowerDNS understands. You are encouraged to implement the section
 * called "addDomainKey", as you can use pdnsutil to provision keys.
 * Query example: {"method": "getdomainkeys", "parameters": {"name":"example.com."}}
 * Response example:
    {
        "result": [
            {
                "id": 1,
                "flags": 256,
                "active": true,
                "content": "<omitted>"
            }
        ]
    }
 */
#[get("/getdomainkeys/<domain>/<domain_id>")]
pub fn dns_get_domain_keys(domain: String, domain_id: i32) -> &'static str {
    "Mockup"
}
