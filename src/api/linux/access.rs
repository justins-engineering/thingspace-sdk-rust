use crate::api::request_helpers::{
  BASE64_BUF_SIZE, LOGIN_URL, M2M_REST_API_V1, encode_login_field, oauth_field,
};
use crate::models::{LoginResponse, Session, SessionRequestBody};
use const_format::concatcp;

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
  public_key: &'a str,
  private_key: &'a str,
  response: &'a mut LoginResponse,
) -> Result<&'a LoginResponse, Box<dyn std::error::Error>> {
  let mut enc_buf = [0u8; BASE64_BUF_SIZE];
  let auth = encode_login_field(public_key, private_key, &mut enc_buf)
    .expect("Failed to encode login field");
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
  cred: &'a SessionRequestBody,
  access_token: &'a str,
  response: &'a mut Session,
) -> Result<&'a Session, Box<dyn std::error::Error>> {
  *response = ureq::post(concatcp!(M2M_REST_API_V1, "/session/login"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", oauth_field(access_token))
    .send_json(cred)?
    .body_mut()
    .read_json::<Session>()?;

  Ok(response)
}
