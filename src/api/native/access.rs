use crate::api::request_helpers::{
  BASE64_BUF_SIZE, LOGIN_URL, M2M_REST_API_V1, encode_login_field, oauth_field,
};
use crate::models::{Error, LoginResponse, Session, SessionRequestBody};
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
/// use serde::{Deserialize, Serialize};
/// use std::fs;
/// use thingspace_sdk::api::get_access_token;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// #[allow(dead_code)]
/// pub struct Secrets {
///   pub public_key: String,
///   pub private_key: String,
///   pub username: String,
///   pub password: String,
///   pub account_name: String,
/// }
///
/// async fn access_token() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let client = reqwest::Client::new();
///
///   match thingspace_sdk::api::get_access_token(
///     &secrets.public_key,
///     &secrets.private_key,
///     Some(client.clone()),
///   )
///   .await
///   {
///     Ok(response) => {
///       println!(
///         "Access token: {}, Scope: {}, TokenType: {}, Expires in: {}",
///         response.access_token, response.scope, response.token_type, response.expires_in
///       );
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub async fn get_access_token(
  public_key: &str,
  private_key: &str,
  client: Option<reqwest::Client>,
) -> Result<LoginResponse, Error> {
  let mut enc_buf = [0u8; BASE64_BUF_SIZE];
  let auth = encode_login_field(public_key, private_key, &mut enc_buf)
    .expect("Failed to encode login field");
  let auth = std::str::from_utf8(auth)?.trim_end_matches('\0');

  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(LOGIN_URL)
    .header("Accept", "application/json")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .header("Authorization", auth)
    .body("grant_type=client_credentials")
    .send()
    .await;

  match request {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response.json::<LoginResponse>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}

/// Makes an API request for a M2M session token and returns a [`Session`].
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use std::fs;
/// use thingspace_sdk::api::get_session_token;
/// use thingspace_sdk::models::SessionRequestBody;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// #[allow(dead_code)]
/// pub struct Secrets {
///   pub public_key: String,
///   pub private_key: String,
///   pub username: String,
///   pub password: String,
///   pub account_name: String,
/// }
///
/// async fn session_token(access_token: &str) {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let client = reqwest::Client::new();
///
///   let user_info = SessionRequestBody {
///     username: secrets.username.clone(),
///     password: secrets.password.clone(),
///   };
///
///   match thingspace_sdk::api::get_session_token(&user_info, access_token, Some(client)).await {
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
pub async fn get_session_token(
  cred: &SessionRequestBody,
  access_token: &str,
  client: Option<reqwest::Client>,
) -> Result<Session, Error> {
  let body = serde_json::to_string(cred)?;
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(concatcp!(M2M_REST_API_V1, "/session/login"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
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
      Ok(response.json::<Session>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}
