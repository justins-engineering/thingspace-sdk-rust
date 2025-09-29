use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thingspace_sdk::models::{Error, LoginResponse, Session};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

#[derive(Serialize, Deserialize, Debug)]
struct RequestHeaders {
  #[serde(rename(serialize = "Accept"))]
  accept: String,
  #[serde(rename(serialize = "Content-Type"))]
  content_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Access {
  access_token: String,
}

pub async fn fetch_access_token() -> Result<LoginResponse, Error> {
  let mut location = String::with_capacity(128);
  location.push_str(&::web_sys::window().unwrap().location().origin().unwrap());
  location.push_str("/api/access_token");

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "".to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers)?;

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);

  let request = Request::new_with_str_and_init(&location, &request_init)?;

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await?;

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(response.json()?).await?;

  serde_wasm_bindgen::from_value(json).map_err(Error::from)
}

pub async fn fetch_session_token() -> Result<Session, Error> {
  let access = Access {
    access_token: use_context::<crate::LocalSession>()
      .access_token
      .read()
      .access_token
      .clone(),
  };

  let access = serde_json::to_string(&access)?;

  let mut location = String::with_capacity(128);
  location.push_str(&::web_sys::window().unwrap().location().origin().unwrap());
  location.push_str("/api/session_token");

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "application/json".to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers)?;

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);
  request_init.set_body(&serde_wasm_bindgen::to_value(&access)?);

  let request = Request::new_with_str_and_init(&location, &request_init)?;

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await?;

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(response.json()?).await?;

  serde_wasm_bindgen::from_value(json).map_err(Error::from)
}
