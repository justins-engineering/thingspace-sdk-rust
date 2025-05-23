#![warn(missing_docs)]
//! A Verizon ThingSpace API library using [`ureq`] as an HTTP client and [`base64ct`] for Base64 encoding.
//!
//! This library currently only covers the NBIoT related API endpoints.

use base64ct::{Base64, Encoding};
use const_format::concatcp;
use serde::{Deserialize, Serialize};
use std::str;

/// Functions for use with "Device Management" API endpoints, primarily `/devices/actions/list`
pub mod devices;

/// Functions for use with "Registered Callbacks Listeners" API endpoints
pub mod registered_callback_listeners;

const AUTH_BEARER: &str = "Bearer ";
const M2M_REST_API_V1: &str = "https://thingspace.verizon.com/api/m2m/v1";
const AUTH_BUF_SIZE: usize = 64;
const SESSION_TOKEN_FIELD: &str = "VZ-M2M-Token";

fn oauth_field(access_token: &str) -> String {
  let mut auth = String::with_capacity(AUTH_BUF_SIZE);
  auth.push_str(AUTH_BEARER);
  auth.push_str(access_token);

  auth
}

/// A struct containing a user's Verizon account secrets required for API use.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Secrets {
  public_key: String,
  private_key: String,
  username: String,
  password: String,
  account_name: String,
}

/// A struct containing the deserialized JSON returned from an OAuth2 access token API request.
#[derive(Deserialize)]
pub struct LoginResponse {
  /// The OAuth2 access token.
  pub access_token: String,
  /// The OAuth2 access token scope.
  pub scope: String,
  /// The OAuth2 access token type.
  pub token_type: String,
  /// The OAuth2 access TTL.
  pub expires_in: i32,
}

impl Default for LoginResponse {
  fn default() -> LoginResponse {
    LoginResponse {
      access_token: String::with_capacity(64),
      scope: String::with_capacity(64),
      token_type: String::with_capacity(16),
      expires_in: 0,
    }
  }
}

const AUTH_BASIC: &[u8] = b"Basic ";
const LOGIN_BUF_SIZE: usize = 96;
const BASE64_BUF_SIZE: usize = 128;
const LOGIN_URL: &str = "https://thingspace.verizon.com/api/ts/v1/oauth2/token";

fn encode_login_field<'a>(
  secrets: &'a Secrets,
  dst: &'a mut [u8],
) -> Result<&'a [u8], Box<dyn std::error::Error>> {
  let mut login_buf = [0u8; LOGIN_BUF_SIZE];
  assert!(
    secrets.public_key.len() + secrets.private_key.len() + 2 <= LOGIN_BUF_SIZE,
    "LOGIN_BUF_SIZE is too small!"
  );

  let dec_len = secrets.public_key.len() + secrets.private_key.len();

  let (key, value) = login_buf.split_at_mut(secrets.public_key.len());
  key.copy_from_slice(secrets.public_key.as_bytes());
  value[0] = b':';
  let value = &mut value[1..=secrets.private_key.len()];
  value.copy_from_slice(secrets.private_key.as_bytes());

  assert!(
    Base64::encoded_len(&login_buf[..=dec_len]) + AUTH_BASIC.len() <= BASE64_BUF_SIZE,
    "BASE64_BUF_SIZE is too small!"
  );

  let (key, value) = dst.split_at_mut(AUTH_BASIC.len());
  key.copy_from_slice(AUTH_BASIC);
  Base64::encode(&login_buf[..=dec_len], value)?;

  Ok(dst)
}

/// Makes an API request for an OAuth2 access token and returns a [`LoginResponse`].
/// # Panics
/// The [OAuth2 access token request](https://thingspace.verizon.com/documentation/api-documentation.html#/http/quick-start/credentials-and-tokens/obtaining-an-access_token)
/// requires a partially Base64 encoded header field in the form of:
/// ```text
/// Authorization: Basic Base64_encoded(public_key:private_key)
/// ```
/// Because both of these keys *should* be 36 characters long and Base64 encoding size is
/// deterministic we use static buffers to hold both the pre and post encoded header field.
/// ```
/// const LOGIN_BUF_SIZE: usize = 96;
/// const BASE64_BUF_SIZE: usize = 128;
/// ```
/// There are two assertions that verify the pre and post encoded header field will fit into their
/// respective buffers. If the `public_key` or `private_key` are substantially larger, the
/// assertions will fail and cause a panic.
///
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::{Secrets, LoginResponse};
///
/// fn access_token() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///
///   match thingspace_sdk::get_access_token(&secrets, &mut login) {
///     Ok(response) => {
///       println!(
///         "Access token: {}, Scope: {}, TokenType: {}, Expires in: {}",
///         response.access_token, response.scope, response.token_type, response.expires_in
///       );
///     },
///     Err(error) => { println!("{error:?}"); }
///   }
/// }
/// ```
pub fn get_access_token<'a>(
  secrets: &'a Secrets,
  response: &'a mut LoginResponse,
) -> Result<&'a LoginResponse, Box<dyn std::error::Error>> {
  let mut enc_buf = [0u8; BASE64_BUF_SIZE];
  let auth = encode_login_field(secrets, &mut enc_buf).expect("Failed to encode login field");
  let auth = std::str::from_utf8(auth)?.trim_end_matches('\0');

  *response = ureq::post(LOGIN_URL)
    .header("Accept", "application/json")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .header("Authorization", auth)
    .send("grant_type=client_credentials")?
    .body_mut()
    .read_json::<LoginResponse>()?;

  Ok(response)
}

#[derive(Serialize)]
struct SessionRequestBody {
  username: String,
  password: String,
}

/// A struct containing a session access and it's TTL.
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Session {
  /// The session token.
  pub session_token: String,
  /// The session token TTL.
  /// "The token will remain valid as long as your application continues to use it, but it will expire after 20 minutes of inactivity."
  pub expires_in: i32,
}

impl Default for Session {
  fn default() -> Session {
    Session {
      session_token: String::with_capacity(64),
      expires_in: 1200,
    }
  }
}

/// Makes an API request for a M2M session token and returns a [`Session`].
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::{Secrets, LoginResponse, Session};
///
/// fn session_token() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///
///   match thingspace_sdk::get_session_token(&secrets, &login.access_token, &mut session) {
///     Ok(response) => {
///       println!(
///         "Session token: {}, Expires in: {}",
///         response.session_token, response.expires_in
///       );
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub fn get_session_token<'a>(
  secrets: &'a Secrets,
  access_token: &'a str,
  response: &'a mut Session,
) -> Result<&'a Session, Box<dyn std::error::Error>> {
  let request: SessionRequestBody = SessionRequestBody {
    username: secrets.username.clone(),
    password: secrets.password.clone(),
  };

  *response = ureq::post(concatcp!(M2M_REST_API_V1, "/session/login"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", oauth_field(access_token))
    .send_json(&request)?
    .body_mut()
    .read_json::<Session>()?;

  Ok(response)
}
