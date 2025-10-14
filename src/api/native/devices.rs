use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{AccountDeviceListRequest, AccountDeviceListResponse, Error};
use const_format::concatcp;

/// Makes an API request for an Account Device List and returns a [`AccountDeviceListResponse`].
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::{Secrets, LoginResponse, Session};
/// use thingspace_sdk::devices::{AccountDeviceListRequest, AccountDeviceListResponse, devices_list};
///
/// fn device_list() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let mut device_request = AccountDeviceListRequest::default();
///   let mut device_result = AccountDeviceListResponse::default();
///
///   match devices_list(
///     &secrets,
///     &login.access_token,
///     &session.session_token,
///     &mut device_request,
///     &mut device_result,
///   ) {
///     Ok(response) => {
///       println!("{:?}", response.devices[0]);
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub async fn devices_list(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  adl: &mut AccountDeviceListRequest,
  client: Option<reqwest::Client>,
) -> Result<AccountDeviceListResponse, Error> {
  adl.account_name = Some(account_name.to_string());

  let body = serde_json::to_string(adl)?;
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(concatcp!(M2M_REST_API_V1, "/devices/actions/list"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header(SESSION_TOKEN_FIELD, session_token)
    .header("Authorization", oauth_field(access_token))
    .body(body)
    .send()
    .await;

  match request {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response.json::<AccountDeviceListResponse>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}
