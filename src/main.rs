use std::fs;
use thingspace_sdk::devices::{AccountDeviceListRequest, AccountDeviceListResult, devices_list};
use thingspace_sdk::registered_callback_listeners::{
  CallbackListener, CallbackListenerResponse, deregister_callback_listener,
  list_callback_listeners, register_callback_listener,
};
use thingspace_sdk::{LoginResponse, Secrets, Session};

fn read_secrets_from_file() -> Result<Secrets, Box<dyn std::error::Error>> {
  let file = fs::read_to_string("./secrets.toml")?;
  let secrets = toml::from_str::<Secrets>(&file)?;
  Ok(secrets)
}

struct Credentials {
  access_token: String,
  session_token: String,
}

fn main() {
  let secrets = read_secrets_from_file().expect("Failed to read from secrets.toml");

  let mut credentials = Credentials {
    access_token: String::with_capacity(64),
    session_token: String::with_capacity(64),
  };

  get_credentials(&secrets, &mut credentials);
  get_devices(&secrets, &mut credentials);

  set_callback_listener(&secrets, &mut credentials);
  print_listeners(&secrets, &mut credentials);
  delete_callback_listener(&secrets, &mut credentials);
  print_listeners(&secrets, &mut credentials);
}

fn get_credentials(secrets: &Secrets, cred: &mut Credentials) {
  let mut login = LoginResponse::default();

  match thingspace_sdk::get_access_token(secrets, &mut login) {
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

  let mut session = Session::default();

  match thingspace_sdk::get_session_token(secrets, &login.access_token, &mut session) {
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

fn get_devices(secrets: &Secrets, cred: &mut Credentials) {
  let mut device_request = AccountDeviceListRequest::default();
  let mut device_result = AccountDeviceListResult::default();

  match devices_list(
    secrets,
    &cred.access_token,
    &cred.session_token,
    &mut device_request,
    &mut device_result,
  ) {
    Ok(response) => {
      println!("{:#?}", response.devices[0]);
    }
    Err(error) => {
      println!("{error:?}");
    }
  }
}

fn set_callback_listener(secrets: &Secrets, cred: &mut Credentials) {
  let mut rcl = CallbackListener {
    service_name: "CarrierService".to_string(),
    url: "https://mock.thingspace.verizon.com/webhook".to_string(),
    ..Default::default()
  };

  let mut response = CallbackListenerResponse::default();

  match register_callback_listener(
    secrets,
    &cred.access_token,
    &cred.session_token,
    &mut rcl,
    &mut response,
  ) {
    Ok(_) => {
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

fn delete_callback_listener(secrets: &Secrets, cred: &mut Credentials) {
  let service_name = "CarrierService".to_string();

  let mut response = CallbackListenerResponse::default();

  match deregister_callback_listener(
    secrets,
    &cred.access_token,
    &cred.session_token,
    &service_name,
    &mut response,
  ) {
    Ok(_) => {
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

fn print_listeners(secrets: &Secrets, cred: &mut Credentials) {
  let mut rcls = vec![CallbackListener {
    account_name: Some(String::with_capacity(16)),
    ..Default::default()
  }];

  match list_callback_listeners(secrets, &cred.access_token, &cred.session_token, &mut rcls) {
    Ok(_) => {
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
