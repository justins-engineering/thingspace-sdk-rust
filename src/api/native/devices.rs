use crate::api::request_helpers::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, oauth_field};
use crate::models::{
  AccountDeviceListRequest, AccountDeviceListResponse, Error, NiddMessage, NiddRequest,
};
use const_format::concatcp;

/// Makes an API request for an Account Device List and returns a [`AccountDeviceListResponse`].
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use thingspace_sdk::api::devices_list;
/// use thingspace_sdk::models::AccountDeviceListRequest;
///
/// async fn print_devices(account_name: &str, access_token: &str, session_token: &str) {
///   let mut request = AccountDeviceListRequest::default();
///
///   match devices_list(account_name, access_token, session_token, &mut request, None).await {
///     Ok(response) => {
///       for device in response.devices {
///         println!("{:?}", device.device_ids);
///       }
///     }
///     Err(error) => println!("{error:?}"),
///   }
/// }
/// ```
pub async fn devices_list(
  account_name: &str,
  access_token: &str,
  session_token: &str,
  adl: &mut AccountDeviceListRequest,
  client: Option<reqwest::Client>,
) -> Result<AccountDeviceListResponse, Error> {
  adl.account_name = Some(account_name.to_string());

  let body = serde_json::to_string(adl)?;
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(concatcp!(M2M_REST_API_V1, "/devices/actions/list"))
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
      Ok(response.json::<AccountDeviceListResponse>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}

pub async fn send_nidd(
  access_token: &str,
  session_token: &str,
  nidd_msg: &mut NiddMessage,
  client: Option<reqwest::Client>,
) -> Result<NiddRequest, Error> {
  let body = serde_json::to_string(nidd_msg)?;
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(concatcp!(M2M_REST_API_V1, "/devices/nidd/message"))
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
      Ok(response.json::<NiddRequest>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}
