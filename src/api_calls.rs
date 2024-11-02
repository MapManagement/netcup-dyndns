use crate::api_objects::*;

const LOGIN_URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";
const SUCCESS_STATUS_CODE: i32 = 2000;
const LOGIN_ACTION: &str = "login";

pub async fn create_login_session(api_login: CredentialsFile) -> Result<ResponseMessage, i32> {
    let login_request = ApiRequest {
        action: LOGIN_ACTION.to_string(),
        param: api_login,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(LOGIN_URL)
        .json(&login_request)
        .send()
        .await
        .expect("Couldn't log into Netcup API.")
        .json::<ResponseMessage>()
        .await
        .expect("Couldn't parse response of Netcup API.");

    match response.statuscode {
        SUCCESS_STATUS_CODE => Ok(response),
        _ => Err(response.statuscode),
    }
}

pub async fn update_dns_records(auth: ApiAuth) {}

pub async fn update_dns_zone(auth: ApiAuth) {}

pub async fn update_domain(auth: ApiAuth) {}

pub async fn update_handle(auth: ApiAuth) {}
