use crate::api::request_helpers::{LOGIN_URL, M2M_REST_API_V1, basic_auth_field, oauth_field};
use crate::models::{Error, LoginResponse, Session, SessionRequestBody};
use const_format::concatcp;

/// Makes an API request for an OAuth2 access token and returns a [`LoginResponse`].
///
/// The [OAuth2 access token request](https://thingspace.verizon.com/documentation/api-documentation.html#/http/quick-start/credentials-and-tokens/obtaining-an-access_token)
/// sends `Authorization: Basic` and the Base64 of `public_key:private_key`, sized from the keys,
/// so no key length can fail or panic.
///
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use std::fs;
/// use thingspace_sdk::api::get_access_token;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// #[allow(dead_code)]
/// pub struct Secrets {
///   pub public_key: String,
///   pub private_key: String,
///   pub username: String,
///   pub password: String,
///   pub account_name: String,
/// }
///
/// async fn access_token() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let client = reqwest::Client::new();
///
///   match thingspace_sdk::api::get_access_token(
///     &secrets.public_key,
///     &secrets.private_key,
///     Some(client.clone()),
///   )
///   .await
///   {
///     Ok(response) => println!("Access token expires in {} s", response.expires_in),
///     Err(error) => println!("{error:?}"),
///   }
/// }
/// ```
pub async fn get_access_token(
  public_key: &str,
  private_key: &str,
  client: Option<reqwest::Client>,
) -> Result<LoginResponse, Error> {
  let auth = basic_auth_field(public_key, private_key);

  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(LOGIN_URL)
    .header("Accept", "application/json")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .header("Authorization", auth)
    .body("grant_type=client_credentials")
    .send()
    .await;

  match request {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        let json = response.json().await?;
        return Err(Error::ThingSpace(json));
      }
      Ok(response.json::<LoginResponse>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}

/// Makes an API request for a M2M session token and returns a [`Session`].
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use std::fs;
/// use thingspace_sdk::api::get_session_token;
/// use thingspace_sdk::models::SessionRequestBody;
///
/// #[derive(Serialize, Deserialize, Debug, Clone)]
/// #[allow(dead_code)]
/// pub struct Secrets {
///   pub public_key: String,
///   pub private_key: String,
///   pub username: String,
///   pub password: String,
///   pub account_name: String,
/// }
///
/// async fn session_token(access_token: &str) {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let client = reqwest::Client::new();
///
///   let user_info = SessionRequestBody {
///     username: secrets.username.clone(),
///     password: secrets.password.clone(),
///   };
///
///   match thingspace_sdk::api::get_session_token(&user_info, access_token, Some(client)).await {
///     Ok(_session) => println!("Session started"),
///     Err(error) => println!("{error:?}"),
///   }
/// }
/// ```
pub async fn get_session_token(
  cred: &SessionRequestBody,
  access_token: &str,
  client: Option<reqwest::Client>,
) -> Result<Session, Error> {
  let body = serde_json::to_string(cred)?;
  let client = match client {
    Some(c) => c,
    None => reqwest::Client::new(),
  };

  let request = client
    .post(concatcp!(M2M_REST_API_V1, "/session/login"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
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
      Ok(response.json::<Session>().await?)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}
