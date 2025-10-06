use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{AccountDeviceListRequest, AccountDeviceListResponse, Secrets};
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
pub fn devices_list<'a>(
  account_name: &'a str,
  access_token: &'a str,
  session_token: &'a str,
  request: &'a mut AccountDeviceListRequest,
  response: &'a mut AccountDeviceListResponse,
) -> Result<&'a AccountDeviceListResponse, Box<dyn std::error::Error>> {
  request.account_name.clone_from(account_name);

  *response = ureq::post(concatcp!(M2M_REST_API_V1, "/devices/actions/list"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header(SESSION_TOKEN_FIELD, session_token)
    .header("Authorization", oauth_field(access_token))
    .send_json(request)?
    .body_mut()
    .read_json::<AccountDeviceListResponse>()?;

  Ok(response)
}
