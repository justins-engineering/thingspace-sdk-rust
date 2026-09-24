use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct containing the deserialized JSON returned from an OAuth2 access token API request.
///
/// It has no `Display`, and its `Debug` redacts the token, so logging one cannot leak it.
#[derive(Clone, Deserialize, Serialize)]
pub struct LoginResponse {
  /// The OAuth2 access token.
  pub access_token: String,
  /// The OAuth2 access token scope.
  pub scope: String,
  /// The OAuth2 access token type.
  pub token_type: String,
  /// The OAuth2 access TTL.
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
