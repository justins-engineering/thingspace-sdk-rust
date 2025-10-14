use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thingspace_sdk::models::{AccountDeviceListResponse, Error};
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

pub async fn device_list() {
  let mut location = String::with_capacity(128);
  let host = &::web_sys::window()
    .unwrap()
    .location()
    .origin()
    .unwrap_throw();

  location.push_str(host);
  location.push_str("/api/device");

  let headers = RequestHeaders {
    accept: "application/json".to_string(),
    content_type: "application/json".to_string(),
  };

  let headers = serde_wasm_bindgen::to_value(&headers);
  let Ok(headers) = headers else {
    error!("Failed to set fetch headers!");
    return;
  };

  let request_init = RequestInit::new();
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);

  let request = Request::new_with_str_and_init(&location, &request_init);
  let Ok(request) = request else {
    error!("Failed to create devices request!");
    return;
  };

  let resp_value = JsFuture::from(
    web_sys::window()
      .unwrap_throw()
      .fetch_with_request(&request),
  )
  .await;
  let Ok(resp_value) = resp_value else {
    error!("Failed to read devices request!");
    return;
  };

  assert!(resp_value.is_instance_of::<Response>());
  let response: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(response.json().unwrap_throw()).await;
  let Ok(json) = json else {
    error!("Failed to parse devices response json!");
    return;
  };

  match serde_wasm_bindgen::from_value::<AccountDeviceListResponse>(json).map_err(Error::from) {
    Ok(resp) => {
      use_context::<crate::LocalSession>()
        .devices
        .set(resp.devices);
    }
    Err(e) => {
      error!("{e}");
    }
  };
}
