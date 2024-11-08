use crate::api_objects::*;
use crate::configuration::*;

const API_URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";
const SUCCESS_STATUS_CODE: i32 = 2000;
const LOGIN_ACTION: &str = "login";

pub async fn create_login_session(api_login: &Credentials) -> Result<LoginResponse, i32> {
    let login_request = ApiRequest {
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
// ==== read information ====

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

    println!("{:?}", domain_info_request);

    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .json(&domain_info_request)
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

// ==== update information ====

pub async fn update_dns_records() {}

pub async fn update_dns_zone() {}

pub async fn update_domain() {}

pub async fn update_handle() {}
