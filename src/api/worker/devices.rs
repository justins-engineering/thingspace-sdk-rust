use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{AccountDeviceListRequest, Error, NiddMessage};
use const_format::concatcp;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, console_error};

/// Makes an API request for an Account Device List and returns the
/// [`AccountDeviceListResponse`] in a `worker::Response`.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use crate::cache;
/// use thingspace_sdk::api::devices_list;
/// use thingspace_sdk::models::AccountDeviceListRequest;
/// use worker::{Request, Response, RouteContext, console_error};
///
/// pub async fn list_devices(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
///   let atoken = cache::access_token(&ctx).await?;
///   let stoken = cache::session_token(&ctx).await?;
///   let aname = ctx.var("ACCOUNT_NAME")?;
///
///   let adl = AccountDeviceListRequest {
///     account_name: Some(aname.to_string()),
///     device_id: None,
///     filter: None,
///     current_state: None,
///     earliest: None,
///     latest: None,
///     service_plan: None,
///     max_number_of_devices: None,
///     largest_device_id_seen: None,
///   };
///
///   let vz_req = devices_list(&atoken, &stoken, &adl).await;
///
///   match vz_req {
///     Ok(resp) => Ok(resp),
///     Err(e) => {
///       console_error!("{e}");
///       Response::error(e.to_string(), 500)
///     }
///   }
/// }
/// ```
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

pub async fn send_nidd(
  access_token: &str,
  session_token: &str,
  nidd_msg: &mut NiddMessage,
) -> Result<Response, Error> {
  let headers = Headers::new();
  headers.append("Accept", "application/json")?;
  headers.append("Content-Type", "application/json")?;
  headers.append("Authorization", &oauth_field(access_token))?;
  headers.append(SESSION_TOKEN_FIELD, session_token)?;

  let body = serde_json::to_string(nidd_msg)?;

  let mut request_init = RequestInit::new();
  request_init.with_method(Method::Post);

  request_init.with_headers(headers);
  request_init.with_body(Some(serde_wasm_bindgen::to_value(&body)?));

  let request = Request::new_with_init(
    concatcp!(M2M_REST_API_V1, "/devices/nidd/message"),
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
