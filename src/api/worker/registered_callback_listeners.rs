use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{CallbackListener, Error};
use const_format::concatcp;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, console_error};

/// Registers a given URL as a callback listener for the given [`CallbackListener::service_name`] and account.
/// # Errors
/// Returns `Error::ThingSpace()` on responses with status code 400..600
/// Returns `Error::Worker()` on a failed Fetch request
///
/// # Example
/// ```rust
/// use crate::cache;
/// use thingspace_sdk::api::register_callback_listener;
/// use thingspace_sdk::models::CallbackListener;
/// use worker::{Request, Response, RouteContext, console_error};
///
/// pub async fn create_listeners(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
///   let ctype: Result<Option<String>, worker::Error> = req.headers().get("Content-Type");
///
///   let Ok(ctype) = ctype else {
///     return Response::error("Missing 'Content-Type' header", 400);
///   };
///   let Some(ctype) = ctype else {
///     return Response::error("Bad 'Content-Type' header", 400);
///   };
///
///   if ctype == "application/json" {
///     let Ok(cbl) = req.json::<CallbackListener>().await else {
///       return Response::error("Request missing 'CallbackListener'", 400);
///     };
///
///     if cbl.service_name.is_empty() {
///       return Response::error("CallbackListener missing 'service_name'", 400);
///     }
///
///     if cbl.url.is_empty() {
///       return Response::error("CallbackListener missing 'url'", 400);
///     }
///
///     let aname = ctx.var("ACCOUNT_NAME")?;
///     let atoken = cache::access_token(&ctx).await?;
///     let stoken = cache::session_token(&ctx).await?;
///     let vz_req = register_callback_listener(&aname.to_string(), &atoken, &stoken, &cbl).await;
///
///     match vz_req {
///       Ok(resp) => Ok(resp),
///       Err(e) => {
///         console_error!("{:?}", e);
///         Response::error(e.to_string(), 500)
///       }
///     }
///   } else {
///     Response::error("Wrong 'Content-Type' header", 400)
///   }
/// }
/// ```
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
/// # Errors
/// Returns `Error::ThingSpace()` on responses with status code 400..600
/// Returns `Error::Worker()` on a failed Fetch request
///
/// # Example
/// ```rust
/// use crate::cache;
/// use thingspace_sdk::api::deregister_callback_listener;
/// use worker::{Request, Response, RouteContext, console_error};
///
/// pub async fn delete_listeners(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
///   if let Some(sname) = ctx.param("name") {
///     let aname = ctx.var("ACCOUNT_NAME")?;
///     let atoken = cache::access_token(&ctx).await?;
///     let stoken = cache::session_token(&ctx).await?;
///
///     let vz_req = deregister_callback_listener(&aname.to_string(), &atoken, &stoken, sname).await;
///
///     match vz_req {
///       Ok(resp) => Ok(resp),
///       Err(e) => {
///         console_error!("{:?}", e);
///         Response::error(e.to_string(), 500)
///       }
///     }
///   } else {
///     Response::error("Missing 'service_name' parameter in url", 400)
///   }
/// }
/// ```
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
/// # Errors
/// Returns `Error::ThingSpace()` on responses with status code 400..600
/// Returns `Error::Worker()` on a failed Fetch request
///
/// # Example
/// ```rust
/// use crate::cache;
/// use thingspace_sdk::api::list_callback_listeners;
/// use worker::{Request, Response, RouteContext, console_error};
///
/// pub async fn list_listeners(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
///   let aname = ctx.var("ACCOUNT_NAME")?;
///   let atoken = cache::access_token(&ctx).await?;
///   let stoken = cache::session_token(&ctx).await?;
///
///   let vz_req = list_callback_listeners(&aname.to_string(), &atoken, &stoken).await;
///
///   match vz_req {
///     Ok(resp) => Ok(resp),
///     Err(e) => {
///       console_error!("{:?}", e);
///       Response::error(e.to_string(), 500)
///     }
///   }
/// }
/// ```
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
