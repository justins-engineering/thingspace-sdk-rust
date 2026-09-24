use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct containing the deserialized JSON returned from an OAuth2 access token API request.
///
/// Only `access_token` is required: Verizon does not document the other fields, and a response
/// that omits one must not fail the login.
///
/// It has no `Display`, and its `Debug` redacts the token, so logging one cannot leak it.
#[derive(Clone, Deserialize, Serialize)]
pub struct LoginResponse {
  /// The OAuth2 access token.
  pub access_token: String,
  /// The OAuth2 access token scope, empty when the response omits it.
  #[serde(default)]
  pub scope: String,
  /// The OAuth2 access token type, empty when the response omits it.
  #[serde(default)]
  pub token_type: String,
  /// The OAuth2 access TTL in seconds, 3600 when the response omits it.
  #[serde(default = "one_hour")]
  pub expires_in: i32,
}

/// The access token's lifetime as Verizon documents it, for a response that omits `expires_in`.
fn one_hour() -> i32 {
  3600
}

impl Default for LoginResponse {
  fn default() -> LoginResponse {
    LoginResponse {
      access_token: String::with_capacity(64),
      scope: String::with_capacity(64),
      token_type: String::with_capacity(16),
      expires_in: one_hour(),
    }
  }
}

impl fmt::Debug for LoginResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("LoginResponse")
      .field("access_token", &"<redacted>")
      .field("scope", &self.scope)
      .field("token_type", &self.token_type)
      .field("expires_in", &self.expires_in)
      .finish()
  }
}

#[cfg(test)]
mod tests {
  use super::LoginResponse;

  #[test]
  fn a_full_token_response_parses() {
    let body = r#"{"access_token":"d7bc43e9acc31aba9654fc5cd0d8c520",
      "scope":"am_application_scope default","token_type":"Bearer","expires_in":3599}"#;

    let login: LoginResponse = serde_json::from_str(body).unwrap();
    assert_eq!(login.access_token, "d7bc43e9acc31aba9654fc5cd0d8c520");
    assert_eq!(login.scope, "am_application_scope default");
    assert_eq!(login.token_type, "Bearer");
    assert_eq!(login.expires_in, 3599);
  }

  #[test]
  fn a_token_response_needs_only_the_token() {
    let body = r#"{"access_token":"d7bc43e9acc31aba9654fc5cd0d8c520"}"#;

    let login: LoginResponse = serde_json::from_str(body).unwrap();
    assert_eq!(login.access_token, "d7bc43e9acc31aba9654fc5cd0d8c520");
    assert!(login.scope.is_empty());
    assert!(login.token_type.is_empty());
    assert_eq!(login.expires_in, 3600);
  }

  #[test]
  fn a_token_response_without_the_token_fails() {
    let body = r#"{"scope":"default","token_type":"Bearer","expires_in":3600}"#;

    assert!(serde_json::from_str::<LoginResponse>(body).is_err());
  }

  #[test]
  fn debug_redacts_the_access_token() {
    let login = LoginResponse {
      access_token: "d7bc43e9acc31aba9654fc5cd0d8c520".to_string(),
      expires_in: 3600,
      ..Default::default()
    };

    let debug = format!("{login:?}");
    assert!(!debug.contains("d7bc43e9acc31aba9654fc5cd0d8c520"));
    assert!(debug.contains("expires_in: 3600"));
  }
}
