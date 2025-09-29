use serde::{Deserialize, Serialize};

/// A struct containing a registered callback listener.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CallbackListener {
  #[serde(rename(serialize = "name"))]
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

/// A struct containing an Account Device List Result.
#[derive(Debug, Deserialize)]
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
