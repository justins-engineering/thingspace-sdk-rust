use std::fs;
use thingspace_sdk::devices::{AccountDeviceListRequest, AccountDeviceListResult, devices_list};
use thingspace_sdk::{LoginResponse, Secrets, Session};

fn read_secrets_from_file() -> Result<Secrets, Box<dyn std::error::Error>> {
  let file = fs::read_to_string("./secrets.toml")?;
  let secrets = toml::from_str::<Secrets>(&file)?;
  Ok(secrets)
}

fn main() {
  let secrets = read_secrets_from_file().expect("Failed to read from secrets.toml");
  let mut login = LoginResponse::default();

  match thingspace_sdk::get_access_token(&secrets, &mut login) {
    Ok(response) => {
      println!(
        "Access token: {}, Scope: {}, TokenType: {}, Expires in: {}",
        response.access_token, response.scope, response.token_type, response.expires_in
      );
    }
    Err(error) => {
      println!("{error:?}");
    }
  }

  let mut session = Session::default();

  match thingspace_sdk::get_session_token(&secrets, &login.access_token, &mut session) {
    Ok(response) => {
      println!(
        "Session token: {}, Expires in: {}",
        response.session_token, response.expires_in
      );
    }
    Err(error) => {
      println!("{error:?}");
    }
  }

  let mut device_request = AccountDeviceListRequest::default();
  let mut device_result = AccountDeviceListResult::default();

  match devices_list(
    &secrets,
    &login.access_token,
    &session.session_token,
    &mut device_request,
    &mut device_result,
  ) {
    Ok(response) => {
      println!("{:?}", response.devices[0]);
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}
