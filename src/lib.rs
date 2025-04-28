#![warn(missing_docs)]
//! A Verizon ThingSpace API library using [`ureq`] as an HTTP client and [`base64ct`] for Base64 encoding.
//!
//! This library currently only covers the NBIoT related API endpoints.

use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use std::str;
use ureq::{http::response, Error};

// #[derive(Serialize)]
// struct MySendBody {
//    thing: String,
// }

/// A struct containing a user's Verizon account secrets required for API use.
#[derive(Deserialize)]
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

const LOGIN_BUF_SIZE: usize = 96;
const BASE64_BUF_SIZE: usize = 128;
const AUTH_KEY: &[u8] = b"Basic ";
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
        Base64::encoded_len(&login_buf[..=dec_len]) + AUTH_KEY.len() <= BASE64_BUF_SIZE,
        "BASE64_BUF_SIZE is too small!"
    );

    let (key, value) = dst.split_at_mut(AUTH_KEY.len());
    key.copy_from_slice(b"Basic ");
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
/// fn read_secrets_from_file() {
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
