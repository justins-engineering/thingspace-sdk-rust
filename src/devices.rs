use super::{M2M_REST_API_V1, SESSION_TOKEN_FIELD, Secrets, oauth_field};
use const_format::concatcp;
use iso8601::DateTime;
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
#[derive(Serialize)]
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

/// A struct containing an Account Device List Request.
#[derive(Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountDeviceListRequest {
  /// The billing account for which a list of devices is returned.
  /// If you don't specify an accountName, the list includes all devices to which you have access.
  account_name: String,
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

impl Default for AccountDeviceListRequest {
  fn default() -> AccountDeviceListRequest {
    AccountDeviceListRequest {
      account_name: String::with_capacity(32),
      device_id: Option::default(),
      filter: Option::default(),
      current_state: Option::default(),
      earliest: Option::default(),
      latest: Option::default(),
      service_plan: Option::default(),
      max_number_of_devices: Option::default(),
      largest_device_id_seen: Option::default(),
    }
  }
}

/// A struct containing information about the device's carrier.
#[derive(Debug, Deserialize)]
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

/// Any extended attributes for the device, as Key and Value pairs.
/// The pairs listed below are returned as part of the response for a single device,
/// but are not included if the request was for information about multiple devices.
#[derive(Debug, Deserialize, Clone)]
#[serde(default, rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ExtendedAttribute {
  /// Extended Attribute key
  pub key: String,
  /// Extended Attribute value, usually empty
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

/// A struct containing a Device.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Device {
  /// Account name
  pub account_name: String,
  /// Billing cycle end date
  pub billing_cycle_end_date: DateTime,
  /// Array of [`CarrierInformation`] objects (should only conatain 1 object)
  pub carrier_informations: [CarrierInformation; 1],
  /// Connection state
  pub connected: bool,
  /// Device creation date
  pub created_at: DateTime,
  /// Array of [`DeviceID`]s
  pub device_ids: [DeviceID; 6],
  /// Array of [`ExtendedAttribute`]s
  pub extended_attributes: Vec<ExtendedAttribute>,
  /// Array of device group names (should only contain 1 String, the default group name)
  pub group_names: [String; 1],
  /// Last activated by user
  pub last_activation_by: String,
  /// Last activation date
  pub last_activation_date: DateTime,
  /// Last connection date
  pub last_connection_date: DateTime,
}

impl Default for Device {
  fn default() -> Device {
    Device {
      account_name: String::with_capacity(32),
      billing_cycle_end_date: DateTime::default(),
      carrier_informations: [CarrierInformation::default()],
      connected: bool::default(),
      created_at: DateTime::default(),
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
      last_activation_date: DateTime::default(),
      last_connection_date: DateTime::default(),
    }
  }
}

/// A struct containing an Account Device List Result.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AccountDeviceListResult {
  /// Indicates if more devices exist than were returned in the request, see [`AccountDeviceListRequest::max_number_of_devices`]
  /// > False for a status 200 response.True for a status 202 response, indicating that there is more data to be retrieved.
  pub has_more_data: bool,
  /// Array of returned [`Device`] objects
  pub devices: Vec<Device>,
}

impl Default for AccountDeviceListResult {
  fn default() -> AccountDeviceListResult {
    AccountDeviceListResult {
      has_more_data: bool::default(),
      devices: vec![Device::default()],
    }
  }
}

/// Makes an API request for an Account Device List and returns a [`AccountDeviceListResult`].
/// # Errors
/// Returns HTTP response code or `std::error::Error`.
///
/// # Example
/// ```rust
/// use std::fs;
/// use thingspace_sdk::{Secrets, LoginResponse, Session};
/// use thingspace_sdk::devices::{AccountDeviceListRequest, AccountDeviceListResult, devices_list};
///
/// fn device_list() {
///   let file = fs::read_to_string("./secrets.toml").unwrap();
///   let secrets = toml::from_str::<Secrets>(&file).expect("Failed to read from secrets.toml");
///   let mut login = LoginResponse::default();
///   let mut session = Session::default();
///   let mut device_request = AccountDeviceListRequest::default();
///   let mut device_result = AccountDeviceListResult::default();
///
///   match devices_list(
///     &secrets,
///     &login.access_token,
///     &session.session_token,
///     &mut device_request,
///     &mut device_result,
///   ) {
///     Ok(response) => {
///       println!("{:?}", response.devices[0]);
///     }
///     Err(error) => {
///       println!("{error:?}");
///     }
///   }
/// }
/// ```
pub fn devices_list<'a>(
  secrets: &Secrets,
  access_token: &'a str,
  session_token: &'a str,
  request: &'a mut AccountDeviceListRequest,
  response: &'a mut AccountDeviceListResult,
) -> Result<&'a AccountDeviceListResult, Box<dyn std::error::Error>> {
  request.account_name.clone_from(&secrets.account_name);

  *response = ureq::post(concatcp!(M2M_REST_API_V1, "/devices/actions/list"))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header(SESSION_TOKEN_FIELD, session_token)
    .header("Authorization", oauth_field(access_token))
    .send_json(request)?
    .body_mut()
    .read_json::<AccountDeviceListResult>()?;

  Ok(response)
}
