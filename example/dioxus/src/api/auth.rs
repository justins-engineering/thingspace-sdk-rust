use dioxus::logger::tracing::error;
use serde::{Deserialize, Serialize};
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

pub async fn login() {
  let mut location = String::with_capacity(128);
  location.push_str(&::web_sys::window().unwrap().location().origin().unwrap());
  location.push_str("/browser/access_token");

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);

  let request = Request::new_with_str_and_init(&location, &request_init);

  let Ok(request) = request else {
    error!("Failed to create access token request!");
    return;
  };

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await;

  let Ok(resp_value) = resp_value else {
    error!("Failed to read access token request!");
    return;
  };

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  if !response.ok() {
    error!("Access request failed");
    return;
  }

  let mut location = String::with_capacity(128);
  location.push_str(&::web_sys::window().unwrap().location().origin().unwrap());
  location.push_str("/browser/session_token");

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);

  let request = Request::new_with_str_and_init(&location, &request_init);

  let Ok(request) = request else {
    error!("Failed to create session token request!");
    return;
  };

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await;

  let Ok(resp_value) = resp_value else {
    error!("Failed to read session token request!");
    return;
  };

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  if !response.ok() {
    error!("Session request failed");
  }
}
