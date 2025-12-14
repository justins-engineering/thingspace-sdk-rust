use crate::models::devices::DeviceID;
use serde::{Deserialize, Serialize};

/// A struct containing a NIDD Message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NiddMessage {
  /// The name of a billing account, in the form of 10 digits, a hyphen,
  /// and then five more digits. Must include any leading zeros.
  pub account_name: String,
  /// Array of [`DeviceID`]s
  pub device_ids: Vec<DeviceID>,
  /// Identifies the maximum time for the delivery of the data to the device,
  /// in units of seconds. The allowed range is between 2 secs and 2592000 secs (30 days).
  pub maximum_delivery_time: i32,
  /// A base64-encoded binary message. The maximum size of the data can be 10864 bits or 1358 bytes.
  pub message: String,
}

impl Default for NiddMessage {
  fn default() -> NiddMessage {
    NiddMessage {
      account_name: String::with_capacity(32),
      device_ids: vec![DeviceID::default()],
      maximum_delivery_time: i32::default(),
      message: String::default(),
    }
  }
}
