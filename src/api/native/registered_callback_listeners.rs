use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{CallbackListener, CallbackListenerResponse, Error};
use const_format::concatcp;

/// Registers a given URL as a callback listener for the given [`CallbackListener::service_name`] and account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::registered_callback_listeners::{CallbackListener, CallbackListenerResponse, register_callback_listener};
/// use thingspace_sdk::{LoginResponse, Secrets, Session};
///
///
/// fn set_callback_listener() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let mut rcl = CallbackListener {
///     service_name: "CarrierService".to_string(),
///     url: "https://mock.thingspace.verizon.com/webhook".to_string(),
///     ..Default::default()
///   };
///
///   let mut response = CallbackListenerResponse::default();
///
///   match register_callback_listener(&secrets, &login.access_token, &session.session_token, &mut rcl, &mut response) {
///     Ok(_) => {
///       println!(
///         "Account: {}\nService: {}\n",
///         response.account_name,
///         response.service_name,
///       );
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub async fn register_callback_listener(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  cbl: &CallbackListener,
  client: Option<reqwest::Client>,
) -> Result<CallbackListenerResponse, Error> {
  let body = serde_json::to_string(cbl)?;
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let mut url = String::with_capacity(80);
  url.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  url.push_str(account_name);

  let request = client
    .post(url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header(SESSION_TOKEN_FIELD, session_token)
    .header("Authorization", oauth_field(access_token))
    .body(body)
    .send()
    .await;

  match request {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response.json::<CallbackListenerResponse>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}

/// Removes a registered callback listener for the given [`CallbackListener::service_name`] and account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::registered_callback_listeners::{CallbackListenerResponse, deregister_callback_listener};
/// use thingspace_sdk::{LoginResponse, Secrets, Session};
///
///
/// fn delete_callback_listener() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let service_name = "CarrierService".to_string();
///
///   let mut response = CallbackListenerResponse::default();
///
///   match deregister_callback_listener(&secrets, &login.access_token, &session.session_token, &service_name, &mut response) {
///     Ok(_) => {
///       println!(
///         "Account: {}\nService: {}\n",
///         response.account_name,
///         response.service_name,
///       );
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub async fn deregister_callback_listener(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  service_name: &str,
  client: Option<reqwest::Client>,
) -> Result<CallbackListenerResponse, Error> {
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let mut url = String::with_capacity(128);
  url.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  url.push_str(account_name);
  url.push_str("/name/");
  url.push_str(service_name);

  let request = client
    .delete(url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header(SESSION_TOKEN_FIELD, session_token)
    .header("Authorization", oauth_field(access_token))
    .send()
    .await;

  match request {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response.json::<CallbackListenerResponse>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }

  // *response = ureq::delete(url)
  //   .header("Accept", "application/json")
  //   .header("Content-Type", "application/json")
  //   .header("Authorization", oauth_field(access_token))
  //   .header(SESSION_TOKEN_FIELD, session_token)
  //   .call()?
  //   .body_mut()
  //   .read_json::<CallbackListenerResponse>()?;

  // Ok(response)
}

/// Returns the name and endpoint URL of the callback listening services registered for a given account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::registered_callback_listeners::{CallbackListener, list_callback_listeners};
/// use thingspace_sdk::{LoginResponse, Secrets, Session};
///
///
/// fn print_listeners() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let mut rcls = vec![CallbackListener {
///     account_name: Some(String::with_capacity(16)),
///     ..Default::default()
///   }];
///
///   match list_callback_listeners(&secrets, &login.access_token, &session.session_token, &mut rcls) {
///     Ok(_) => {
///       for rcl in rcls {
///         println!(
///           "\n Account-name: {}\nService: {}\nurl: {}\n",
///           rcl.account_name.unwrap(),
///           rcl.service_name,
///           rcl.url
///         );
///       }
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub async fn list_callback_listeners(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  client: Option<reqwest::Client>,
) -> Result<Vec<CallbackListener>, Error> {
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let mut url = String::with_capacity(80);
  url.push_str(concatcp!(M2M_REST_API_V1, "/callbacks/"));
  url.push_str(account_name);

  let request = client
    .get(url)
    .header("Accept", "application/json")
    .header(SESSION_TOKEN_FIELD, session_token)
    .header("Authorization", oauth_field(access_token))
    .send()
    .await;

  match request {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response.json::<Vec<CallbackListener>>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }

  // *response = ureq::get(url)
  //   .header("Accept", "application/json")
  //   .header("Authorization", oauth_field(access_token))
  //   .header(SESSION_TOKEN_FIELD, session_token)
  //   .call()?
  //   .body_mut()
  //   .read_json::<Vec<CallbackListener>>()?;
}
