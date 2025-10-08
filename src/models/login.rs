use serde::{Deserialize, Serialize};

/// A struct containing the deserialized JSON returned from an OAuth2 access token API request.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl std::fmt::Display for LoginResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Access Token: {}\nExpires: {}\nScope: {}\nToken Type: {}",
      self.access_token, self.expires_in, self.scope, self.token_type
    )
  }
}
