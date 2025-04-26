use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use std::str;
use ureq::{Error, http::response};

// #[derive(Serialize)]
// struct MySendBody {
//    thing: String,
// }

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Secrets {
  public_key: String,
  private_key: String,
  username: String,
  password: String,
  account_name: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct LoginResponse {
  pub access_token: String,
  pub scope: String,
  pub token_type: String,
  pub expires_in: i32,
}

impl Default for LoginResponse {
  fn default() -> LoginResponse {
    LoginResponse {
      access_token: String::with_capacity(64),
      scope: String::with_capacity(64),
      token_type: String::with_capacity(16),
      expires_in: 0,
    }
  }
}

const BASE64_BUF_SIZE: usize = 128;
const LOGIN_BUF_SIZE: usize = 96;
const AUTH_KEY: &[u8] = b"Basic ";
const LOGIN_URL: &str = "https://thingspace.verizon.com/api/ts/v1/oauth2/token";

fn encode_login_field<'a>(
  secrets: &'a Secrets,
  dst: &'a mut [u8],
) -> Result<&'a [u8], Box<dyn std::error::Error>> {
  let mut login_buf = [0u8; LOGIN_BUF_SIZE];
  assert!(
    secrets.public_key.len() + secrets.private_key.len() + 2 <= LOGIN_BUF_SIZE,
    "LOGIN_BUF_SIZE is too small!"
  );

  let dec_len = secrets.public_key.len() + secrets.private_key.len();

  let (key, value) = login_buf.split_at_mut(secrets.public_key.len());
  key.copy_from_slice(secrets.public_key.as_bytes());
  value[0] = b':';
  let value = &mut value[1..=secrets.private_key.len()];
  value.copy_from_slice(secrets.private_key.as_bytes());

  assert!(
    Base64::encoded_len(&login_buf[..=dec_len]) + AUTH_KEY.len() <= BASE64_BUF_SIZE,
    "BASE64_BUF_SIZE is too small!"
  );

  let (key, value) = dst.split_at_mut(AUTH_KEY.len());
  key.copy_from_slice(b"Basic ");
  Base64::encode(&login_buf[..=dec_len], value)?;

  Ok(dst)
}

pub fn get_access_token<'a>(
  secrets: &'a Secrets,
  response: &'a mut LoginResponse,
) -> Result<&'a LoginResponse, Box<dyn std::error::Error>> {
  let mut enc_buf = [0u8; BASE64_BUF_SIZE];
  let auth = encode_login_field(secrets, &mut enc_buf).expect("Failed to encode login field");
  let auth = std::str::from_utf8(auth)?.trim_end_matches('\0');

  *response = ureq::post(LOGIN_URL)
    .header("Accept", "application/json")
    .header("Content-Type", "application/x-www-form-urlencoded")
    .header("Authorization", auth)
    .send("grant_type=client_credentials")?
    .body_mut()
    .read_json::<LoginResponse>()?;

  Ok(response)
}
