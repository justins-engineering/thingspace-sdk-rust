use const_format::concatcp;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, console_error};

use crate::api::request_helpers::{LOGIN_URL, M2M_REST_API_V1, basic_auth_field, oauth_field};
use crate::models::{Error, SessionRequestBody};

/// Makes an API request for an OAuth2 access token; the success body is a
/// [`LoginResponse`](crate::models::LoginResponse).
///
/// The [OAuth2 access token request](https://thingspace.verizon.com/documentation/api-documentation.html#/http/quick-start/credentials-and-tokens/obtaining-an-access_token)
/// sends `Authorization: Basic` and the Base64 of `public_key:private_key`, sized from the keys,
/// so no key length can fail or panic.
///
/// # Errors
/// Returns HTTP response code or `thingspace_sdk::Error`.
pub async fn get_access_token(
  public_key: &str,
  private_key: &str,
) -> std::result::Result<Response, Error> {
  let auth = basic_auth_field(public_key, private_key);

  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/x-www-form-urlencoded")?;
  headers.append("Authorization", &auth)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Post);
  // request_init.set_mode(RequestMode::Cors);
  // request_init.set_credentials(RequestCredentials::Include);

  request_init.with_headers(headers);
  request_init.with_body(Some(wasm_bindgen::JsValue::from_str(
    "grant_type=client_credentials",
  )));

  let request: Request = Request::new_with_init(LOGIN_URL, &request_init)?;

  match Fetch::Request(request).send().await {
    Ok(mut response) => {
      let status = response.status_code();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::Credential(json));
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
  cred: &SessionRequestBody,
  access_token: &str,
) -> std::result::Result<Response, Error> {
  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;

  let cred = serde_json::to_string(&cred)?;

  let mut request_init: RequestInit = RequestInit::new();
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
        return Err(Error::Credential(json));
      }
      Ok(response)
    }
    Err(e) => {
      console_error!("{:?}", e);
      Err(Error::Worker(e))
    }
  }
}
