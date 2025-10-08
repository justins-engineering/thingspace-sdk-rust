use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionRequestBody {
  pub username: String,
  pub password: String,
}

/// A struct containing a session access and it's TTL.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Session {
  /// The session token.
  pub session_token: String,
  /// The session token TTL.
  /// The token will remain valid as long as your application continues to use it,
  /// but it will expire after 20 minutes of inactivity.
  pub expires_in: i32,
}

impl Default for Session {
  fn default() -> Session {
    Session {
      session_token: String::with_capacity(64),
      expires_in: 1200,
    }
  }
}

impl std::fmt::Display for Session {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Session Token: {}\nExpires: {}",
      self.session_token, self.expires_in
    )
  }
}
