use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thingspace_sdk::models::Error;
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

#[cfg(feature = "browser")]
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

#[cfg(feature = "api")]
pub async fn login() {
  let mut location = String::with_capacity(128);
  let host = &::web_sys::window()
    .unwrap()
    .location()
    .origin()
    .unwrap_throw();

  location.push_str(host);
  location.push_str("/api/access_token");

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
  request_init.set_method("POST");
  request_init.set_mode(RequestMode::Cors);
  request_init.set_credentials(RequestCredentials::SameOrigin);
  request_init.set_headers(&headers);

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

  let json = JsFuture::from(response.json().unwrap_throw()).await;
  let Ok(json) = json else {
    error!("Failed to parse access token response json!");
    return;
  };
  let mut access = use_context::<crate::LocalSession>().access_token;

  match serde_wasm_bindgen::from_value(json).map_err(Error::from) {
    Ok(acc_resp) => {
      access.set(acc_resp);
    }
    Err(e) => {
      error!("{e}");
    }
  };

  let access_token = Access {
    access_token: access.read().access_token.clone(),
  };

  let access = serde_json::to_string(&access_token);
  let Ok(access) = access else {
    error!("Failed to read access token context!");
    return;
  };

  let mut location = String::with_capacity(128);
  location.push_str(host);
  location.push_str("/api/session_token");

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
  request_init.set_body(&serde_wasm_bindgen::to_value(&access).unwrap_throw());

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

  let json = JsFuture::from(response.json().unwrap_throw()).await;
  let Ok(json) = json else {
    error!("Failed to parse session token response json!");
    return;
  };

  match serde_wasm_bindgen::from_value(json).map_err(Error::from) {
    Ok(acc_resp) => {
      use_context::<crate::LocalSession>()
        .session_token
        .set(acc_resp);
    }
    Err(e) => {
      error!("{e}");
    }
  };
}

// pub async fn fetch_session_token() -> Result<Session, Error> {
//   let access = Access {
//     access_token: use_context::<crate::LocalSession>()
//       .access_token
//       .read()
//       .access_token
//       .clone(),
//   };

//   let access = serde_json::to_string(&access)?;

//   let mut location = String::with_capacity(128);
//   location.push_str(&::web_sys::window().unwrap().location().origin().unwrap());
//   location.push_str("/browser/session_token");

//   let headers = RequestHeaders {
//     accept: "application/json".to_string(),
//     content_type: "application/json".to_string(),
//   };

//   let headers = serde_wasm_bindgen::to_value(&headers)?;

//   let request_init = RequestInit::new();
//   request_init.set_method("POST");
//   request_init.set_mode(RequestMode::Cors);
//   request_init.set_credentials(RequestCredentials::SameOrigin);
//   request_init.set_headers(&headers);
//   request_init.set_body(&serde_wasm_bindgen::to_value(&access)?);

//   let request = Request::new_with_str_and_init(&location, &request_init)?;

//   let resp_value = JsFuture::from(
//     web_sys::window()
//       .unwrap_throw()
//       .fetch_with_request(&request),
//   )
//   .await?;

//   assert!(resp_value.is_instance_of::<Response>());
//   let response: Response = resp_value.dyn_into().unwrap();

//   let json = JsFuture::from(response.json()?).await?;

//   serde_wasm_bindgen::from_value(json).map_err(Error::from)
// }

// pub async fn fetch_session_token() -> Result<Session, Error> {
//   let access = Access {
//     access_token: use_context::<crate::LocalSession>()
//       .access_token
//       .read()
//       .access_token
//       .clone(),
//   };

//   let access = serde_json::to_string(&access)?;

//   let mut location = String::with_capacity(128);
//   location.push_str(&::web_sys::window().unwrap().location().origin().unwrap());
//   location.push_str("/browser/session_token");

//   let headers = RequestHeaders {
//     accept: "application/json".to_string(),
//     content_type: "application/json".to_string(),
//   };

//   let headers = serde_wasm_bindgen::to_value(&headers)?;

//   let request_init = RequestInit::new();
//   request_init.set_method("POST");
//   request_init.set_mode(RequestMode::Cors);
//   request_init.set_credentials(RequestCredentials::SameOrigin);
//   request_init.set_headers(&headers);
//   request_init.set_body(&serde_wasm_bindgen::to_value(&access)?);

//   let request = Request::new_with_str_and_init(&location, &request_init)?;

//   let resp_value = JsFuture::from(
//     web_sys::window()
//       .unwrap_throw()
//       .fetch_with_request(&request),
//   )
//   .await?;

//   assert!(resp_value.is_instance_of::<Response>());
//   let response: Response = resp_value.dyn_into().unwrap();

//   let json = JsFuture::from(response.json()?).await?;

//   serde_wasm_bindgen::from_value(json).map_err(Error::from)
// }
