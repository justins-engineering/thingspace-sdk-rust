use dioxus::logger::tracing::{error, info, warn};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thingspace_sdk::models::{CallbackListener, CallbackListenerResponse, Error};
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

pub async fn listener_list() {
  let mut location = String::with_capacity(128);
  let host = &::web_sys::window()
    .unwrap()
    .location()
    .origin()
    .unwrap_throw();

  location.push_str(host);
  location.push_str("/api/callback_listener");

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "".to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers);
  let Ok(headers) = headers else {
    error!("Failed to set fetch headers!");
    return;
  };

  let request_init = RequestInit::new();
  request_init.set_method("GET");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);

  let request = Request::new_with_str_and_init(&location, &request_init);
  let Ok(request) = request else {
    error!("Failed to create listeners request!");
    return;
  };

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await;
  let Ok(resp_value) = resp_value else {
    error!("Failed to read listeners request!");
    return;
  };

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(response.json().unwrap_throw()).await;
  let Ok(json) = json else {
    error!("Failed to parse listeners response json!");
    return;
  };
  let mut list = use_context::<crate::LocalSession>().listeners;

  match serde_wasm_bindgen::from_value(json).map_err(Error::from) {
    Ok(resp) => {
      list.set(resp);
    }
    Err(e) => {
      error!("{e}");
    }
  };
}

pub async fn create_listener(listener: &CallbackListener) {
  let mut location = String::with_capacity(128);
  let host = &::web_sys::window()
    .unwrap()
    .location()
    .origin()
    .unwrap_throw();

  location.push_str(host);
  location.push_str("/api/callback_listener");

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "application/json".to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers);
  let Ok(headers) = headers else {
    error!("Failed to set fetch headers!");
    return;
  };

  let body = serde_json::to_string(listener);
  let Ok(body) = body else {
    error!("Failed to serialize CallbackListener!");
    return;
  };

  let body = serde_wasm_bindgen::to_value(&body);
  let Ok(body) = body else {
    error!("Failed to convert CallbackListener to JsValue!");
    return;
  };

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);
  request_init.set_body(&body);

  let request = Request::new_with_str_and_init(&location, &request_init);
  let Ok(request) = request else {
    error!("Failed to create listeners request!");
    return;
  };

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await;
  let Ok(resp_value) = resp_value else {
    error!("Failed to read listeners request!");
    return;
  };

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(response.json().unwrap_throw()).await;
  let Ok(json) = json else {
    error!("Failed to parse listeners response json!");
    return;
  };

  match serde_wasm_bindgen::from_value::<CallbackListenerResponse>(json).map_err(Error::from) {
    Ok(resp) => {
      warn!("{resp:?}");
      listener_list().await;
    }
    Err(e) => {
      error!("{e}");
    }
  };
}

pub async fn delete_listener(service_name: &str) {
  let mut location = String::with_capacity(128);
  let host = &::web_sys::window()
    .unwrap()
    .location()
    .origin()
    .unwrap_throw();

  location.push_str(host);
  location.push_str("/api/callback_listener/");
  location.push_str(service_name);

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "".to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers);
  let Ok(headers) = headers else {
    error!("Failed to set fetch headers!");
    return;
  };

  let request_init = RequestInit::new();
  request_init.set_method("DELETE");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);

  let request = Request::new_with_str_and_init(&location, &request_init);
  let Ok(request) = request else {
    error!("Failed to create listeners request!");
    return;
  };

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await;
  let Ok(resp_value) = resp_value else {
    error!("Failed to read listeners request!");
    return;
  };

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(response.json().unwrap_throw()).await;
  let Ok(json) = json else {
    error!("Failed to parse listeners response json!");
    return;
  };

  match serde_wasm_bindgen::from_value::<CallbackListenerResponse>(json).map_err(Error::from) {
    Ok(resp) => {
      warn!("{resp:?}");
      listener_list().await;
    }
    Err(e) => {
      error!("{e}");
    }
  };
}
