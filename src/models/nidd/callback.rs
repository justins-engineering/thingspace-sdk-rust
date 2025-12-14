use crate::models::devices::DeviceID;
use crate::models::nidd::NiddResponse;
use serde::Deserialize;

/// A struct containing a NIDD delivery Callback.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NiddCallback {
  /// A unique string that associates the request with the NIDD information that is sent in
  /// asynchronous callback messages. ThingSpace sends a separate callback message for each
  /// device that was in the request. All of the callback messages for an individual query have
  /// the same requestId.
  pub request_id: String,
  /// Array of [`DeviceID`]s
  /// Only one object with {kind,id}, where "kind" shall be the same as the one in the initial request.
  pub device_ids: [DeviceID; 1],
  /// NIDD response enum type; either `NiddMONotificationResponse` or `NiddMONotificationResponse`
  pub nidd_response: NiddResponse,
  /// Valid values include: Delivered, Queued, DeliveryFailed
  pub status: Option<String>,
  /// Total number of callback requests.
  pub callback_count: i32,
  /// Maximum number of callbacks allowed.
  pub max_callback_threshold: i32,
}

impl Default for NiddCallback {
  fn default() -> Self {
    NiddCallback {
      request_id: String::with_capacity(36),
      device_ids: [DeviceID::default()],
      nidd_response: NiddResponse::default(),
      status: None,
      callback_count: i32::default(),
      max_callback_threshold: i32::default(),
    }
  }
}
