use std::fs;
use thingspace_sdk::{Secrets, LoginResponse};

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
    },
    Err(error) => { println!("{error:?}"); }
  }
}
