use super::{Device, DeviceID, DeviceIdSearch};
use iso8601::DateTime;
use serde::{Deserialize, Serialize};

/// A struct containing an Account Device List Request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountDeviceListRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The billing account for which a list of devices is returned.
  /// If you don't specify an accountName, the list includes all devices to which you have access.
  pub account_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// [`DeviceID`]: An identifier for a single device.
  pub device_id: Option<DeviceID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// [`DeviceIdSearch`]: Filter for a list of devices.
  pub filter: Option<DeviceIdSearch>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The name of a device state, to only include devices in that state.
  pub current_state: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Only include devices that were added after this date and time.
  pub earliest: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Only include devices that were added before this date and time.
  pub latest: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Only include devices that have this service plan.
  pub service_plan: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Max number of devices returned for request
  /// Constraints: `>= 0`, `<= 100`
  pub max_number_of_devices: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Constraints: `>= 0`, `<= 100`
  pub largest_device_id_seen: Option<i32>,
}

/// A struct containing an Account Device List Result.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AccountDeviceListResponse {
  /// Indicates if more devices exist than were returned in the request, see [`AccountDeviceListRequest::max_number_of_devices`]
  /// > False for a status 200 response.True for a status 202 response, indicating that there is more data to be retrieved.
  pub has_more_data: bool,
  /// Array of returned [`Device`] objects
  pub devices: Vec<Device>,
}
