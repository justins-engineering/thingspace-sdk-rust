use serde::{Deserialize, Serialize};

/// Any extended attributes for the device, as Key and Value pairs.
/// The pairs listed below are returned as part of the response for a single device,
/// but are not included if the request was for information about multiple devices.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default, rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ExtendedAttribute {
  /// Extended Attribute key
  pub key: String,
  /// Extended Attribute value, usually empty
  /// #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
}

impl Default for ExtendedAttribute {
  fn default() -> ExtendedAttribute {
    ExtendedAttribute {
      key: String::with_capacity(32),
      value: Option::default(),
    }
  }
}
