use crate::models::devices::DeviceID;
use serde::{Deserialize, Serialize};

// use strum::{Display, EnumIter, EnumString};

// /// Valid niddMT delivery Callback statuses
// #[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, EnumString, Serialize)]
// pub enum NiddMtStatus {
//   Delivered,
//   Queued,
//   DeliveryFailed,
// }

/// A struct containing a NIDD delivery Callback.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NiddMtDeliveryCallback {
  /// A unique string that associates the request with the NIDD information that is sent in
  /// asynchronous callback messages. ThingSpace sends a separate callback message for each
  /// device that was in the request. All of the callback messages for an individual query have
  /// the same requestId.
  pub request_id: String,
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

// impl Default for NiddMtDeliveryCallback {
//   fn default() -> NiddMtDeliveryCallback {
//     NiddMtDeliveryCallback {
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
