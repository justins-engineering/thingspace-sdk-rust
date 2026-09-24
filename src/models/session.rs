use serde::{Deserialize, Serialize};
use std::fmt;

/// The UWS credentials `get_session_token` logs in with. Its `Debug` redacts the password.
#[derive(Clone, Deserialize, Serialize)]
pub struct SessionRequestBody {
  /// The UWS username.
  pub username: String,
  /// The UWS password.
  pub password: String,
}

impl fmt::Debug for SessionRequestBody {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("SessionRequestBody")
      .field("username", &self.username)
      .field("password", &"<redacted>")
      .finish()
  }
}

/// A session token and its TTL.
///
/// It has no `Display`, and its `Debug` redacts the token, so logging one cannot leak it.
#[derive(Clone, Deserialize, Serialize)]
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

impl fmt::Debug for Session {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Session")
      .field("session_token", &"<redacted>")
      .field("expires_in", &self.expires_in)
      .finish()
  }
}

#[cfg(test)]
mod tests {
  use super::{Session, SessionRequestBody};

  #[test]
  fn debug_redacts_the_session_token_and_the_password() {
    let session = Session {
      session_token: "bcce3ea6-fe4f-4952-bacf-eadd80718e83".to_string(),
      ..Default::default()
    };
    let credentials = SessionRequestBody {
      username: "zbeeblebrox".to_string(),
      password: "IMgr8".to_string(),
    };

    assert!(!format!("{session:?}").contains("bcce3ea6"));
    let debug = format!("{credentials:?}");
    assert!(!debug.contains("IMgr8"));
    assert!(debug.contains("zbeeblebrox"));
  }
}
