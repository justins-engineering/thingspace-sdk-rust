use super::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, Secrets, oauth_field};
use const_format::concatcp;
use serde::{Deserialize, Serialize};

/// A struct containing a registered callback listener.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CallbackListener {
  #[serde(rename(serialize = "name"))]
  /// The name of the callback service that you want to subscribe to.
  pub service_name: String,
  /// The address on your server where you have enabled a listening service for callback messages.
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The user name that the M2M Platform should return in the callback messages.
  pub username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The password that the M2M Platform should return in the callback messages.
  pub password: Option<String>,
  #[serde(skip_serializing)]
  /// The name of the billing account for which callback messages will be sent.
  pub account_name: Option<String>,
}

impl Default for CallbackListener {
  fn default() -> CallbackListener {
    CallbackListener {
      service_name: String::with_capacity(16),
      url: String::with_capacity(64),
      username: Option::default(),
      password: Option::default(),
      account_name: Option::default(),
    }
  }
}

/// A struct containing an Account Device List Result.
#[derive(Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CallbackListenerResponse {
  /// Account name
  pub account_name: String,
  /// Service name
  pub service_name: String,
}

impl Default for CallbackListenerResponse {
  fn default() -> CallbackListenerResponse {
    CallbackListenerResponse {
      account_name: String::with_capacity(32),
      service_name: String::with_capacity(32),
    }
  }
}

/// Registers a given URL as a callback listener for the given [`CallbackListener::service_name`] and account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::registered_callback_listeners::{CallbackListener, CallbackListenerResponse, register_callback_listener};
/// use thingspace_sdk::{LoginResponse, Secrets, Session};
///
///
/// fn set_callback_listener() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let mut rcl = CallbackListener {
///     service_name: "CarrierService".to_string(),
///     url: "https://mock.thingspace.verizon.com/webhook".to_string(),
///     ..Default::default()
///   };
///
///   let mut response = CallbackListenerResponse::default();
///
///   match register_callback_listener(&secrets, &login.access_token, &session.session_token, &mut rcl, &mut response) {
///     Ok(_) => {
///       println!(
///         "Account: {}\nService: {}\n",
///         response.account_name,
///         response.service_name,
///       );
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub fn register_callback_listener<'a>(
  secrets: &Secrets,
  access_token: &'a str,
  session_token: &'a str,
  request: &'a mut CallbackListener,
  response: &'a mut CallbackListenerResponse,
) -> Result<&'a CallbackListenerResponse, Box<dyn std::error::Error>> {
  let mut url = String::with_capacity(80);
  url.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  url.push_str(&secrets.account_name);

  *response = ureq::post(url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", oauth_field(access_token))
    .header(SESSION_TOKEN_FIELD, session_token)
    .send_json(request)?
    .body_mut()
    .read_json::<CallbackListenerResponse>()?;

  Ok(response)
}

/// Removes a registered callback listener for the given [`CallbackListener::service_name`] and account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::registered_callback_listeners::{CallbackListenerResponse, deregister_callback_listener};
/// use thingspace_sdk::{LoginResponse, Secrets, Session};
///
///
/// fn delete_callback_listener() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let service_name = "CarrierService".to_string();
///
///   let mut response = CallbackListenerResponse::default();
///
///   match deregister_callback_listener(&secrets, &login.access_token, &session.session_token, &service_name, &mut response) {
///     Ok(_) => {
///       println!(
///         "Account: {}\nService: {}\n",
///         response.account_name,
///         response.service_name,
///       );
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub fn deregister_callback_listener<'a>(
  secrets: &Secrets,
  access_token: &'a str,
  session_token: &'a str,
  service_name: &'a str,
  response: &'a mut CallbackListenerResponse,
) -> Result<&'a CallbackListenerResponse, Box<dyn std::error::Error>> {
  let mut url = String::with_capacity(128);
  url.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  url.push_str(&secrets.account_name);
  url.push_str("/name/");
  url.push_str(service_name);

  *response = ureq::delete(url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", oauth_field(access_token))
    .header(SESSION_TOKEN_FIELD, session_token)
    .call()?
    .body_mut()
    .read_json::<CallbackListenerResponse>()?;

  Ok(response)
}

/// Returns the name and endpoint URL of the callback listening services registered for a given account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::registered_callback_listeners::{CallbackListener, list_callback_listeners};
/// use thingspace_sdk::{LoginResponse, Secrets, Session};
///
///
/// fn print_listeners() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let mut rcls = vec![CallbackListener {
///     account_name: Some(String::with_capacity(16)),
///     ..Default::default()
///   }];
///
///   match list_callback_listeners(&secrets, &login.access_token, &session.session_token, &mut rcls) {
///     Ok(_) => {
///       for rcl in rcls {
///         println!(
///           "\n Account-name: {}\nService: {}\nurl: {}\n",
///           rcl.account_name.unwrap(),
///           rcl.service_name,
///           rcl.url
///         );
///       }
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub fn list_callback_listeners<'a>(
  secrets: &Secrets,
  access_token: &'a str,
  session_token: &'a str,
  response: &'a mut Vec<CallbackListener>,
) -> Result<&'a Vec<CallbackListener>, Box<dyn std::error::Error>> {
  let mut url = String::with_capacity(80);
  url.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  url.push_str(&secrets.account_name);

  *response = ureq::get(url)
    .header("Accept", "application/json")
    .header("Authorization", oauth_field(access_token))
    .header(SESSION_TOKEN_FIELD, session_token)
    .call()?
    .body_mut()
    .read_json::<Vec<CallbackListener>>()?;

  Ok(response)
}
