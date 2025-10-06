use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{AccountDeviceListRequest, Error};
use const_format::concatcp;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, console_error};

/// Makes an API request for an Account Device List and returns a [`AccountDeviceListResponse`].
pub async fn devices_list(
  access_token: &str,
  session_token: &str,
  adl: &AccountDeviceListRequest,
) -> std::result::Result<Response, Error> {
  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;
  headers.append(SESSION_TOKEN_FIELD, session_token)?;

  let body = serde_json::to_string(adl)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Post);

  request_init.with_headers(headers);
  request_init.with_body(Some(serde_wasm_bindgen::to_value(&body)?));

  let request = Request::new_with_init(
    concatcp!(M2M_REST_API_V1, "/devices/actions/list"),
    &request_init,
  )?;

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
