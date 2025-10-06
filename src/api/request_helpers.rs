const AUTH_BEARER: &str = "Bearer ";
const AUTH_BUF_SIZE: usize = 64;
const AUTH_BASIC: &[u8] = b"Basic ";

pub const M2M_REST_API_V1: &str = "https://thingspace.verizon.com/api/m2m/v1";
pub const LOGIN_BUF_SIZE: usize = 96;
pub const BASE64_BUF_SIZE: usize = 128;
pub const LOGIN_URL: &str = "https://thingspace.verizon.com/api/ts/v1/oauth2/token";
pub const SESSION_TOKEN_FIELD: &str = "VZ-M2M-Token";

pub fn oauth_field(access_token: &str) -> String {
  let mut auth = String::with_capacity(AUTH_BUF_SIZE);
  auth.push_str(AUTH_BEARER);
  auth.push_str(access_token);

  auth
}

pub fn encode_login_field<'a>(
  public_key: &'a str,
  private_key: &'a str,
  dst: &'a mut [u8],
) -> Result<&'a [u8], Box<dyn std::error::Error>> {
  let mut login_buf = [0u8; LOGIN_BUF_SIZE];
  assert!(
    public_key.len() + private_key.len() + 2 <= LOGIN_BUF_SIZE,
    "LOGIN_BUF_SIZE is too small!"
  );

  let dec_len = public_key.len() + private_key.len();

  let (key, value) = login_buf.split_at_mut(public_key.len());
  key.copy_from_slice(public_key.as_bytes());
  value[0] = b':';
  let value = &mut value[1..=private_key.len()];
  value.copy_from_slice(private_key.as_bytes());

  assert!(
    <base64ct::Base64 as base64ct::Encoding>::encoded_len(&login_buf[..=dec_len])
      + AUTH_BASIC.len()
      <= BASE64_BUF_SIZE,
    "BASE64_BUF_SIZE is too small!"
  );

  let (key, value) = dst.split_at_mut(AUTH_BASIC.len());
  key.copy_from_slice(AUTH_BASIC);
  <base64ct::Base64 as base64ct::Encoding>::encode(&login_buf[..=dec_len], value)?;

  Ok(dst)
}
