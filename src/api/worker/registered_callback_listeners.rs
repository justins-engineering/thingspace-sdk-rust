use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{CallbackListener, Error};
use const_format::concatcp;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, console_error};

/// Registers a given URL as a callback listener for the given [`CallbackListener::service_name`] and account.
pub async fn register_callback_listener(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  cbl: &CallbackListener,
) -> std::result::Result<Response, Error> {
  let mut uri = String::with_capacity(80);
  uri.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  uri.push_str(account_name);

  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;
  headers.append(SESSION_TOKEN_FIELD, session_token)?;

  let body = serde_json::to_string(cbl)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Post);

  request_init.with_headers(headers);
  request_init.with_body(Some(serde_wasm_bindgen::to_value(&body)?));

  let request = Request::new_with_init(&uri, &request_init)?;

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

/// Removes a registered callback listener for the given [`CallbackListener::service_name`] and account.
pub async fn deregister_callback_listener(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  service_name: &str,
) -> std::result::Result<Response, Error> {
  let mut uri = String::with_capacity(128);
  uri.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  uri.push_str(account_name);
  uri.push_str("/name/");
  uri.push_str(service_name);

  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;
  headers.append(SESSION_TOKEN_FIELD, session_token)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Delete);

  request_init.with_headers(headers);

  let request = Request::new_with_init(&uri, &request_init)?;

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

/// Returns the name and endpoint URL of the callback listening services registered for a given account.
pub async fn list_callback_listeners(
  account_name: &str,
  access_token: &str,
  session_token: &str,
) -> std::result::Result<Response, Error> {
  let mut uri = String::with_capacity(80);
  uri.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  uri.push_str(account_name);

  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;
  headers.append(SESSION_TOKEN_FIELD, session_token)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Get);

  request_init.with_headers(headers);

  let request = Request::new_with_init(&uri, &request_init)?;

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
