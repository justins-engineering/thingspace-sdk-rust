use crate::models::devices::DeviceID;
use iso8601::DateTime;
use serde::Deserialize;

/// NIDD response enum type; either `NiddMONotificationResponse` or `NiddMONotificationResponse`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NiddResponse {
  #[serde(rename_all = "camelCase")]
  NiddMONotificationResponse {
    /// The name of a billing account, in the form of 10 digits, a hyphen,
    /// and then five more digits. Must include any leading zeros.
    account_name: String,
    /// Base64-encoded binary message.
    message: String,
    /// Array of [`DeviceID`]s
    /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
    device_ids: Vec<DeviceID>,
  },
  #[serde(rename_all = "camelCase")]
  NiddMTDeliveryResponse {
    /// The name of a billing account, in the form of 10 digits, a hyphen,
    /// and then five more digits. Must include any leading zeros.
    account_name: String,
    /// Identifies the absolute time at which the device receiving data is acknowledged
    /// by Network (SCEF). The format should be aligned with RFC3339,
    /// example: "2017-12-19T16:39:57-08:00" (in UTC passed as a String).
    acknowledge_time: Option<DateTime>,
    /// Identifies the absolute time at which the data send is attempted to the device
    /// for the first time, as device becomes reachable.
    first_attempt_delivery_time: Option<DateTime>,
    /// This displays only if the status is Failed. Valid values include:
    /// Buffered, device not reachable
    /// Timeout, could not deliver data
    /// Unknown
    /// NIDD MT payload exceeds the defined limit
    reason: Option<String>,
    /// Array of [`DeviceID`]s
    /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
    device_ids: Vec<DeviceID>,
  },
}

impl Default for NiddResponse {
  fn default() -> Self {
    NiddResponse::NiddMONotificationResponse {
      account_name: String::with_capacity(36),
      message: String::default(),
      device_ids: vec![],
    }
  }
}
