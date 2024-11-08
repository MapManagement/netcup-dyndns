use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub credentials: Credentials,
    pub domains: HashMap<String, Domain>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub customernumber: i32,
    pub apikey: String,
    pub apipassword: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub domain_name: String,
}
