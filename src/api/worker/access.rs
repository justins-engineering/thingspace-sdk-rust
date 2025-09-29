use const_format::concatcp;
use std::str;
use wasm_bindgen::prelude::*;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, console_error, wasm_bindgen};

use crate::api::request_helpers::{
  BASE64_BUF_SIZE, LOGIN_URL, M2M_REST_API_V1, encode_login_field, oauth_field,
};
use crate::models::{Error, Secrets, SessionRequestBody};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RequestHeaders {
  #[serde(rename(serialize = "Accept"))]
  accept: String,
  #[serde(rename(serialize = "Content-Type"))]
  content_type: String,
  #[serde(rename(serialize = "Authorization"))]
  authorization: String,
}

impl Default for RequestHeaders {
  fn default() -> RequestHeaders {
    RequestHeaders {
      accept: "application/json".to_string(),
      content_type: "application/x-www-form-urlencoded".to_string(),
      authorization: String::with_capacity(64),
    }
  }
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
/// Returns HTTP response code or `thingspace_sdk::Error`.
pub async fn get_access_token(secrets: Secrets) -> std::result::Result<Response, Error> {
  let mut enc_buf = [0u8; BASE64_BUF_SIZE];
  let auth = encode_login_field(&secrets, &mut enc_buf).expect("Failed to encode login field");
  let auth = std::str::from_utf8(auth)?.trim_end_matches('\0');

  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/x-www-form-urlencoded")?;
  headers.append("Authorization", auth)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Post);
  // request_init.set_mode(RequestMode::Cors);
  // request_init.set_credentials(RequestCredentials::Include);

  request_init.with_headers(headers);
  request_init.with_body(Some(JsValue::from_str("grant_type=client_credentials")));

  let request = Request::new_with_init(LOGIN_URL, &request_init)?;

  match Fetch::Request(request).send().await {
    Ok(mut response) => {
      let status = response.status_code();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response)
    }
    Err(e) => {
      console_error!("{:?}", e);
      Err(Error::Worker(e))
    }
  }
}

/// Makes an API request for a M2M session token and returns a [`Session`].
/// # Errors
/// Returns HTTP response code or `thingspace_sdk::Error`.
pub async fn get_session_token(
  cred: SessionRequestBody,
  access_token: &str,
) -> std::result::Result<Response, Error> {
  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;

  let cred = serde_json::to_string(&cred)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Post);

  request_init.with_headers(headers);
  request_init.with_body(Some(serde_wasm_bindgen::to_value(&cred)?));

  let request =
    Request::new_with_init(concatcp!(M2M_REST_API_V1, "/session/login"), &request_init)?;

  match Fetch::Request(request).send().await {
    Ok(mut response) => {
      let status = response.status_code();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response)
    }
    Err(e) => {
      console_error!("{:?}", e);
      Err(Error::Worker(e))
    }
  }
}
