use crate::models::devices::DeviceID;
use serde::{Deserialize, Serialize};

/// A struct containing a NIDD DeliveryResponse.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NiddDeliveryResponse {
  /// Array of [`DeviceID`]s
  /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
  pub device_ids: [DeviceID; 6],
  /// Valid values include: Delivered, Queued, DeliveryFailed
  pub status: String,
  /// Total number of callback requests.
  pub callback_count: i32,
  /// Maximum number of callbacks allowed.
  pub max_callback_threshold: i32,
}

// impl Default for NiddDeliveryResponse {
//   fn default() -> NiddDeliveryResponse {
//     NiddDeliveryResponse {
//       request_id: String::with_capacity(36),
//       device_ids: [
//         DeviceID::default(),
//         DeviceID::default(),
//         DeviceID::default(),
//         DeviceID::default(),
//         DeviceID::default(),
//         DeviceID::default(),
//       ],
//       status: String::with_capacity(14),
//       callback_count: i32::default(),
//       max_callback_threshold: i32::default(),
//     }
//   }
// }
