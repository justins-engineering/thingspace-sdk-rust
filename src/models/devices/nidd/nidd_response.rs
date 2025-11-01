use crate::models::devices::DeviceID;
use iso8601::DateTime;
use serde::Deserialize;

// /// A struct containing a niddResponse object.
// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// #[allow(dead_code)]
// pub struct NiddResponse {
//   /// The name of a billing account, in the form of 10 digits, a hyphen,
//   /// and then five more digits. Must include any leading zeros.
//   pub account_name: String,
//   /// Array of [`DeviceID`]s
//   /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
//   pub device_ids: [DeviceID; 6],
//   /// Valid values include: Delivered, Queued, DeliveryFailed
//   pub status: String,
//   /// Total number of callback requests.
//   pub callback_count: i32,
//   /// Maximum number of callbacks allowed.
//   pub max_callback_threshold: i32,
// }

// impl Default for NiddResponse {
//   fn default() -> NiddResponse {
//     NiddResponse {
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NiddMTDeliveryResponse {
  /// The name of a billing account, in the form of 10 digits, a hyphen,
  /// and then five more digits. Must include any leading zeros.
  pub account_name: String,
  /// Identifies the absolute time at which the device receiving data is acknowledged
  /// by Network (SCEF). The format should be aligned with RFC3339,
  /// example: "2017-12-19T16:39:57-08:00" (in UTC passed as a String).
  pub acknowledge_time: Option<DateTime>,
  /// Identifies the absolute time at which the data send is attempted to the device
  /// for the first time, as device becomes reachable.
  pub first_attempt_delivery_time: Option<DateTime>,
  /// This displays only if the status is Failed. Valid values include:
  /// Buffered, device not reachable
  /// Timeout, could not deliver data
  /// Unknown
  /// NIDD MT payload exceeds the defined limit
  pub reason: Option<String>,
  /// Array of [`DeviceID`]s
  /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
  pub device_ids: [DeviceID; 6],
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NiddMONotificationResponse {
  /// The name of a billing account, in the form of 10 digits, a hyphen,
  /// and then five more digits. Must include any leading zeros.
  pub account_name: String,
  /// Base64-encoded binary message.
  pub message: String,
  /// Array of [`DeviceID`]s
  /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
  pub device_ids: [DeviceID; 6],
}

/// A struct containing a niddResponse object.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NiddResponse {
  NiddMTDeliveryResponse(NiddMTDeliveryResponse),
  NiddMONotificationResponse(NiddMONotificationResponse),
}

// niddResponse: {
//   niddMTDeliveryResponse: {
//     accountName: "0742644905-00001",
//     reason: "Internal error",
//     deviceIds: [
//       { id: "350457799502610", kind: "IMEI" },
//       { id: "4062914013", kind: "MDN" },
//       { id: "14062914013", kind: "MSISDN" },
//       { id: "89148000008531108276", kind: "ICCID" }
//     ]
//   }
// }

// niddResponse: {
//   niddMTDeliveryResponse: {
//     accountName: "0742644905-00001",
//     reason: "No PDN connection",
//     deviceIds: [
//       { id: "350457799502610", kind: "IMEI" },
//       { id: "4062914013", kind: "MDN" },
//       { id: "14062914013", kind: "MSISDN" },
//       { id: "89148000008531108276", kind: "ICCID" }
//     ]
//   }
// }
