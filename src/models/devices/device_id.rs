use serde::{Deserialize, Serialize};

/// A struct containing a Device ID type and ID.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceID {
  /// Device ID
  pub id: String,
  /// The type of the device identifier. Valid types of identifiers are:
  /// ESN (decimal), EID, ICCID (up to 20 digits), IMEI (up to 16 digits), MDN, MEID (hexadecimal), MSISDN.
  pub kind: String,
}

impl Default for DeviceID {
  fn default() -> DeviceID {
    DeviceID {
      id: String::with_capacity(32),
      kind: String::with_capacity(8),
    }
  }
}

/// A struct containing a Device ID type and ID.
#[derive(Serialize, Deserialize)]
pub struct DeviceIdSearch {
  /// The string appears anywhere in the identifer.
  pub contains: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The identifer must start with the specified string.
  pub starts_with: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The identifier must end with the specified string.
  pub ends_with: Option<String>,
  /// The type of the device identifier. Valid types of identifiers are:
  /// ESN (decimal), EID, ICCID (up to 20 digits), IMEI (up to 16 digits), MDN, MEID (hexadecimal), MSISDN.
  pub kind: String,
}

impl Default for DeviceIdSearch {
  fn default() -> DeviceIdSearch {
    DeviceIdSearch {
      contains: String::with_capacity(32),
      starts_with: Option::default(),
      ends_with: Option::default(),
      kind: String::with_capacity(8),
    }
  }
}
