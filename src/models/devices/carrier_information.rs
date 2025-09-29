use serde::{Deserialize, Serialize};

/// A struct containing information about the device's carrier.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CarrierInformation {
  /// Carrier name
  pub carrier_name: String,
  /// Service plan
  pub service_plan: String,
  /// Service state
  pub state: String,
}

impl Default for CarrierInformation {
  fn default() -> CarrierInformation {
    CarrierInformation {
      carrier_name: String::with_capacity(32),
      service_plan: String::with_capacity(32),
      state: String::with_capacity(16),
    }
  }
}
