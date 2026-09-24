use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{AccountDeviceListRequest, Error, NiddMessage};
use const_format::concatcp;
use worker::{Headers, Method, Request, RequestInit, Response};

use super::fetch;

/// Makes an API request for an Account Device List and returns the
/// [`AccountDeviceListResponse`](crate::models::AccountDeviceListResponse) in a `worker::Response`.
/// # Errors
/// [`Error::Api`] for an HTTP error status, `Error::Worker` when the fetch itself failed.
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

  fetch(request).await
}

/// Sends one NIDD message to the devices it names; the success body is a
/// [`NiddRequest`](crate::models::NiddRequest) whose `requestId` ThingSpace's callbacks repeat.
///
/// # Errors
/// [`Error::NiddMessage`] before any request when the message fails
/// [`NiddMessage::validate`], [`Error::Api`] for an HTTP error status, `Error::Worker` when the
/// fetch itself failed.
pub async fn send_nidd(
  access_token: &str,
  session_token: &str,
  nidd_msg: &NiddMessage,
) -> Result<Response, Error> {
  nidd_msg.validate()?;

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

  fetch(request).await
}
