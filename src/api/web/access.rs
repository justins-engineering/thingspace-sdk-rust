use const_format::concatcp;
use serde::{Deserialize, Serialize};
use std::str;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

use crate::api::request_helpers::{
  BASE64_BUF_SIZE, LOGIN_URL, M2M_REST_API_V1, encode_login_field, oauth_field,
};
use crate::models::{LoginResponse, Secrets, Session, SessionRequestBody};
use const_format::concatcp;

/// When debugging your Worker via `wrangler dev`, `wrangler tail`, or from the Workers Dashboard,
/// anything passed to this macro will be printed to the terminal or written to the console.
macro_rules! console_debug {
    ($($t:tt)*) => {
        web_sys::console::debug_1(&format_args!($($t)*).to_string().into())
    }
}

/// When debugging your Worker via `wrangler dev`, `wrangler tail`, or from the Workers Dashboard,
/// anything passed to this macro will be printed to the terminal or written to the console.
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format_args!($($t)*).to_string().into())
    }
}

/// When debugging your Worker via `wrangler dev`, `wrangler tail`, or from the Workers Dashboard,
/// anything passed to this macro will be printed to the terminal or written to the console.
macro_rules! console_warn {
    ($($t:tt)*) => {
        web_sys::console::warn_1(&format_args!($($t)*).to_string().into())
    }
}

/// When debugging your Worker via `wrangler dev`, `wrangler tail`, or from the Workers Dashboard,
/// anything passed to this macro will be printed to the terminal or written to the console.
macro_rules! console_error {
    ($($t:tt)*) => {
        web_sys::console::error_1(&format_args!($($t)*).to_string().into())
    }
}

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
#[cfg(feature = "wasm")]
pub async fn get_access_token(secrets: Secrets) -> Result<LoginResponse, Error> {
  let mut enc_buf = [0u8; BASE64_BUF_SIZE];
  let auth = encode_login_field(&secrets, &mut enc_buf).expect("Failed to encode login field");
  let auth = std::str::from_utf8(auth)?.trim_end_matches('\0');

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "application/x-www-form-urlencoded".to_string(),
    authorization: auth.to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers)?;

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::Include);

  request_init.set_headers(&headers);
  request_init.set_body(&JsValue::from_str("grant_type=client_credentials"));

  let request = Request::new_with_str_and_init(LOGIN_URL, &request_init)?;

  // let resp_value: JsValue =
  //   JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request)).await;

  match JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await
  {
    Ok(res) => {
      console_warn!("{:?}", res);
      assert!(res.is_instance_of::<Response>());
      // let response: Response = res.dyn_into().unwrap_throw();

      let resp_value: Result<Response, wasm_bindgen::JsValue> = res.dyn_into();
      match resp_value {
        Ok(response) => {
          console_warn!("{:?}", response);
          let status = response.status();
          let json = JsFuture::from(response.json()?).await?;

          if (400..600).contains(&status) {
            Err(Error::Js(json))
          } else {
            serde_wasm_bindgen::from_value(json).map_err(Error::from)
          }
        }
        Err(e) => {
          console_error!("{e:?}");
          Err(Error::Js(e))
        }
      }
    }
    Err(e) => {
      console_error!("{e:?}");
      Err(Error::Js(e))
    }
  }
}

/// Makes an API request for a M2M session token and returns a [`Session`].
/// # Errors
/// Returns HTTP response code or `thingspace_sdk::Error`.
#[cfg(feature = "wasm")]
pub async fn get_session_token(secrets: Secrets, access_token: &str) -> Result<Session, Error> {
  let request: SessionRequestBody = SessionRequestBody {
    username: secrets.username.clone(),
    password: secrets.password.clone(),
  };

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "application/json".to_string(),
    authorization: oauth_field(access_token),
  };

  let headers = serde_wasm_bindgen::to_value(&headers)?;

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);

  request_init.set_headers(&headers);
  request_init.set_body(&serde_wasm_bindgen::to_value(&request)?);

  let request =
    Request::new_with_str_and_init(concatcp!(M2M_REST_API_V1, "/session/login"), &request_init)?;

  let resp_value = JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request)).await?;

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  // let local_var_status = response.status();
  let json = JsFuture::from(response.json()?).await?;

  serde_wasm_bindgen::from_value(json).map_err(Error::from)
}
