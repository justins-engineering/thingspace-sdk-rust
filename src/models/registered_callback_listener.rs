use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct containing a registered callback listener. Its `Debug` redacts the password.
#[derive(Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CallbackListener {
  #[serde(rename(serialize = "name"), alias = "name")]
  /// The name of the callback service that you want to subscribe to.
  pub service_name: String,
  /// The address on your server where you have enabled a listening service for callback messages.
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The user name that the M2M Platform should return in the callback messages.
  pub username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The password that the M2M Platform should return in the callback messages.
  pub password: Option<String>,
  #[serde(skip_serializing)]
  /// The name of the billing account for which callback messages will be sent.
  pub account_name: Option<String>,
}

impl Default for CallbackListener {
  fn default() -> CallbackListener {
    CallbackListener {
      service_name: String::with_capacity(16),
      url: String::with_capacity(64),
      username: Option::default(),
      password: Option::default(),
      account_name: Option::default(),
    }
  }
}

impl fmt::Debug for CallbackListener {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("CallbackListener")
      .field("service_name", &self.service_name)
      .field("url", &self.url)
      .field("username", &self.username)
      .field("password", &self.password.as_ref().map(|_| "<redacted>"))
      .field("account_name", &self.account_name)
      .finish()
  }
}

impl fmt::Display for CallbackListener {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let e = match &self.account_name {
      Some(n) => {
        format!(
          "\"account_name\": \"{}\",\n\"service_name\": \"{}\",\n\"url\": \"{}\",\n",
          n, self.service_name, self.url
        )
      }
      None => {
        format!(
          "\"service_name\": \"{}\",\n\"url\": \"{}\",\n",
          self.service_name, self.url
        )
      }
    };
    write!(f, "{{ \"CallbackListener\": {{ {e} }} }}")
  }
}

/// A struct containing an Account Callback Listener Response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CallbackListenerResponse {
  /// Account name
  pub account_name: String,
  /// Service name
  pub service_name: String,
}

impl Default for CallbackListenerResponse {
  fn default() -> CallbackListenerResponse {
    CallbackListenerResponse {
      account_name: String::with_capacity(32),
      service_name: String::with_capacity(32),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::CallbackListener;

  #[test]
  fn debug_redacts_the_password() {
    let listener = CallbackListener {
      username: Some("zbeeblebrox".to_string()),
      password: Some("IMgr8".to_string()),
      ..Default::default()
    };

    let debug = format!("{listener:?}");
    assert!(!debug.contains("IMgr8"));
    assert!(debug.contains("zbeeblebrox"));
    assert!(debug.contains("Some(\"<redacted>\")"));
    assert!(format!("{:?}", CallbackListener::default()).contains("password: None"));
  }
}
