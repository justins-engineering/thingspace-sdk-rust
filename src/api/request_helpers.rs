use base64ct::{Base64, Encoding};

const AUTH_BEARER: &str = "Bearer ";
const AUTH_BUF_SIZE: usize = 64;
const AUTH_BASIC: &str = "Basic ";

pub const M2M_REST_API_V1: &str = "https://thingspace.verizon.com/api/m2m/v1";
pub const LOGIN_URL: &str = "https://thingspace.verizon.com/api/ts/v1/oauth2/token";
pub const SESSION_TOKEN_FIELD: &str = "VZ-M2M-Token";

pub fn oauth_field(access_token: &str) -> String {
  let mut auth = String::with_capacity(AUTH_BUF_SIZE);
  auth.push_str(AUTH_BEARER);
  auth.push_str(access_token);

  auth
}

/// The OAuth2 token request's `Authorization` value: `Basic` and the Base64 of
/// `public_key:private_key`, allocated at the size the keys need.
pub fn basic_auth_field(public_key: &str, private_key: &str) -> String {
  let mut pair = Vec::with_capacity(public_key.len() + 1 + private_key.len());
  pair.extend_from_slice(public_key.as_bytes());
  pair.push(b':');
  pair.extend_from_slice(private_key.as_bytes());

  let mut encoded = vec![0u8; Base64::encoded_len(&pair)];
  let mut auth = String::with_capacity(AUTH_BASIC.len() + encoded.len());
  auth.push_str(AUTH_BASIC);
  // The encoder refuses only a buffer shorter than `encoded_len`, which this one never is.
  auth.push_str(Base64::encode(&pair, &mut encoded).unwrap_or_default());

  auth
}

#[cfg(test)]
mod tests {
  use super::basic_auth_field;

  #[test]
  fn basic_auth_field_encodes_the_key_pair() {
    // RFC 7617's own example.
    assert_eq!(
      basic_auth_field("Aladdin", "open sesame"),
      "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
    );
  }

  #[test]
  fn basic_auth_field_takes_keys_of_any_length() {
    let key = "k".repeat(200);
    let secret = "s".repeat(300);

    let auth = basic_auth_field(&key, &secret);
    assert!(auth.starts_with("Basic "));
    assert_eq!(
      auth.len(),
      "Basic ".len() + (key.len() + 1 + secret.len()).div_ceil(3) * 4
    );
  }
}
