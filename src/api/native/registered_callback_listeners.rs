use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{CallbackListener, CallbackListenerResponse, Error};
use const_format::concatcp;

/// Registers a given URL as a callback listener for the given [`CallbackListener::service_name`] and account.
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use thingspace_sdk::api::register_callback_listener;
/// use thingspace_sdk::models::CallbackListener;
///
/// async fn set_callback_listener(account_name: &str, access_token: &str, session_token: &str) {
///   let listener = CallbackListener {
///     service_name: "CarrierService".to_string(),
///     url: "https://mock.thingspace.verizon.com/webhook".to_string(),
///     ..Default::default()
///   };
///
///   match register_callback_listener(account_name, access_token, session_token, &listener, None)
///     .await
///   {
///     Ok(response) => {
///       println!("Account: {}\nService: {}", response.account_name, response.service_name);
///     }
///     Err(error) => println!("{error:?}"),
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
/// use thingspace_sdk::api::deregister_callback_listener;
///
/// async fn delete_callback_listener(account_name: &str, access_token: &str, session_token: &str) {
///   let service_name = "CarrierService";
///
///   match deregister_callback_listener(
///     account_name,
///     access_token,
///     session_token,
///     service_name,
///     None,
///   )
///   .await
///   {
///     Ok(response) => {
///       println!("Account: {}\nService: {}", response.account_name, response.service_name);
///     }
///     Err(error) => println!("{error:?}"),
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
/// use thingspace_sdk::api::list_callback_listeners;
///
/// async fn print_listeners(account_name: &str, access_token: &str, session_token: &str) {
///   match list_callback_listeners(account_name, access_token, session_token, None).await {
///     Ok(listeners) => {
///       for listener in listeners {
///         println!("Service: {}\nurl: {}", listener.service_name, listener.url);
///       }
///     }
///     Err(error) => println!("{error:?}"),
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
