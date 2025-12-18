use super::{CarrierInformation, DeviceID, ExtendedAttribute};
use serde::{Deserialize, Serialize};
// use iso8601::DateTime;

/// A struct containing a Device.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Device {
  /// Account name
  pub account_name: String,
  /// Billing cycle end date
  pub billing_cycle_end_date: String,
  /// Array of [`CarrierInformation`] objects (should only conatain 1 object)
  pub carrier_informations: [CarrierInformation; 1],
  /// Connection state
  pub connected: bool,
  /// Device creation date
  pub created_at: String,
  /// Array of [`DeviceID`]s
  pub device_ids: [DeviceID; 6],
  /// Array of [`ExtendedAttribute`]s
  pub extended_attributes: Vec<ExtendedAttribute>,
  /// Array of device group names (should only contain 1 String, the default group name)
  pub group_names: [String; 1],
  /// Last activated by user
  pub last_activation_by: String,
  /// Last activation date
  pub last_activation_date: String,
  /// Last connection date
  pub last_connection_date: String,
}

impl Default for Device {
  fn default() -> Device {
    Device {
      account_name: String::with_capacity(32),
      billing_cycle_end_date: String::with_capacity(32),
      carrier_informations: [CarrierInformation::default()],
      connected: bool::default(),
      created_at: String::with_capacity(32),
      device_ids: [
        DeviceID::default(),
        DeviceID::default(),
        DeviceID::default(),
        DeviceID::default(),
        DeviceID::default(),
        DeviceID::default(),
      ],
      extended_attributes: vec![ExtendedAttribute::default(); 26],
      group_names: [String::with_capacity(32)],
      last_activation_by: String::with_capacity(32),
      last_activation_date: String::with_capacity(32),
      last_connection_date: String::with_capacity(32),
    }
  }
}
