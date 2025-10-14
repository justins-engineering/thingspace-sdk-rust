use serde::{Deserialize, Serialize};
use std::fs;
use thingspace_sdk::api::{
  deregister_callback_listener, devices_list, list_callback_listeners, register_callback_listener,
};
use thingspace_sdk::models::{AccountDeviceListRequest, CallbackListener, SessionRequestBody};

/// A struct containing a user's Verizon account secrets required for API use.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Secrets {
  pub public_key: String,
  pub private_key: String,
  pub username: String,
  pub password: String,
  pub account_name: String,
}

fn read_secrets_from_file() -> Result<Secrets, Box<dyn std::error::Error>> {
  let file = fs::read_to_string("./secrets.toml")?;
  let secrets = toml::from_str::<Secrets>(&file)?;
  Ok(secrets)
}

struct Credentials {
  access_token: String,
  session_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let secrets = read_secrets_from_file().expect("Failed to read from secrets.toml");

  let mut credentials = Credentials {
    access_token: String::with_capacity(64),
    session_token: String::with_capacity(64),
  };

  let client = reqwest::Client::new();

  get_credentials(&secrets, &mut credentials, client.clone()).await;
  get_devices(&secrets, &mut credentials, client.clone()).await;

  set_callback_listener(&secrets.account_name, &mut credentials, client.clone()).await;
  print_listeners(&secrets.account_name, &mut credentials, client.clone()).await;
  delete_callback_listener(&secrets.account_name, &mut credentials, client.clone()).await;
  print_listeners(&secrets.account_name, &mut credentials, client).await;
  Ok(())
}

async fn get_credentials(secrets: &Secrets, cred: &mut Credentials, client: reqwest::Client) {
  match thingspace_sdk::api::get_access_token(
    &secrets.public_key,
    &secrets.private_key,
    Some(client.clone()),
  )
  .await
  {
    Ok(response) => {
      println!(
        "Access token: {}, Scope: {}, TokenType: {}, Expires in: {}",
        response.access_token, response.scope, response.token_type, response.expires_in
      );
      cred.access_token.clone_from(&response.access_token);
    }
    Err(error) => {
      println!("{error:?}");
    }
  }

  let user_info = SessionRequestBody {
    username: secrets.username.clone(),
    password: secrets.password.clone(),
  };

  match thingspace_sdk::api::get_session_token(&user_info, &cred.access_token, Some(client)).await {
    Ok(response) => {
      println!(
        "Session token: {}, Expires in: {}",
        response.session_token, response.expires_in
      );
      cred.session_token.clone_from(&response.session_token);
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}

async fn get_devices(secrets: &Secrets, cred: &mut Credentials, client: reqwest::Client) {
  let mut device_request = AccountDeviceListRequest::default();

  match devices_list(
    &secrets.account_name,
    &cred.access_token,
    &cred.session_token,
    &mut device_request,
    Some(client),
  )
  .await
  {
    Ok(response) => {
      println!("{:#?}", response.devices[0]);
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}

async fn set_callback_listener(aname: &str, cred: &mut Credentials, client: reqwest::Client) {
  let rcl = CallbackListener {
    service_name: "CarrierService".to_string(),
    url: "https://mock.thingspace.verizon.com/webhook".to_string(),
    ..Default::default()
  };

  match register_callback_listener(
    aname,
    &cred.access_token,
    &cred.session_token,
    &rcl,
    Some(client),
  )
  .await
  {
    Ok(response) => {
      println!(
        "Account: {}\nService: {}",
        response.account_name, response.service_name,
      );
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}

async fn delete_callback_listener(aname: &str, cred: &mut Credentials, client: reqwest::Client) {
  let service_name = "CarrierService".to_string();

  match deregister_callback_listener(
    aname,
    &cred.access_token,
    &cred.session_token,
    &service_name,
    Some(client),
  )
  .await
  {
    Ok(response) => {
      println!(
        "Account: {}\nService: {}",
        response.account_name, response.service_name,
      );
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}

async fn print_listeners(aname: &str, cred: &mut Credentials, client: reqwest::Client) {
  match list_callback_listeners(aname, &cred.access_token, &cred.session_token, Some(client)).await
  {
    Ok(rcls) => {
      for rcl in rcls {
        println!(
          "Account-name: {}\nService: {}\nurl: {}",
          rcl.account_name.unwrap(),
          rcl.service_name,
          rcl.url
        );
      }
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}
