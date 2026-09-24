use crate::models::devices::DeviceID;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

/// The most data one NIDD message carries, in decoded bytes.
pub const MAX_NIDD_BYTES: usize = 1358;

/// The `maximumDeliveryTime` values ThingSpace accepts, in seconds: 2 s to 30 days.
pub const NIDD_DELIVERY_TIME_SECS: RangeInclusive<i32> = 2..=2_592_000;

/// Why [`NiddMessage::validate`] refused a message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NiddMessageError {
  /// `message` decodes to this many bytes, more than [`MAX_NIDD_BYTES`].
  TooLarge(usize),
  /// `maximum_delivery_time` is outside [`NIDD_DELIVERY_TIME_SECS`].
  DeliveryTime(i32),
}

/// A struct containing a NIDD Message. `send_nidd` refuses one that fails
/// [`validate`](NiddMessage::validate) before sending anything.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NiddMessage {
  /// The name of a billing account, in the form of 10 digits, a hyphen,
  /// and then five more digits. Must include any leading zeros.
  pub account_name: String,
  /// Array of [`DeviceID`]s
  pub device_ids: Vec<DeviceID>,
  /// Identifies the maximum time for the delivery of the data to the device,
  /// in units of seconds. The allowed range is between 2 secs and 2592000 secs (30 days).
  pub maximum_delivery_time: i32,
  /// A base64-encoded binary message. The maximum size of the data can be 10864 bits or 1358 bytes.
  pub message: String,
}

impl NiddMessage {
  /// Checks the two limits ThingSpace documents for a message. The size is counted from
  /// `message`'s length; whether it is valid base64 is left to ThingSpace.
  ///
  /// # Errors
  /// The first limit the message breaks.
  pub fn validate(&self) -> Result<(), NiddMessageError> {
    let bytes = decoded_len(&self.message);
    if bytes > MAX_NIDD_BYTES {
      return Err(NiddMessageError::TooLarge(bytes));
    }
    if !NIDD_DELIVERY_TIME_SECS.contains(&self.maximum_delivery_time) {
      return Err(NiddMessageError::DeliveryTime(self.maximum_delivery_time));
    }
    Ok(())
  }
}

/// Bytes a base64 string decodes to: three per four digits, padding not counted.
fn decoded_len(base64: &str) -> usize {
  let digits = base64.trim_end_matches('=').len();
  digits / 4 * 3 + digits % 4 * 3 / 4
}

impl Default for NiddMessage {
  fn default() -> NiddMessage {
    NiddMessage {
      account_name: String::with_capacity(32),
      device_ids: vec![DeviceID::default()],
      maximum_delivery_time: i32::default(),
      message: String::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{MAX_NIDD_BYTES, NiddMessage, NiddMessageError, decoded_len};
  use crate::models::DeviceID;
  use base64ct::{Base64, Encoding};

  /// Verizon's example request, with `maximumDeliveryTime` an integer as its error table requires.
  fn documented_message() -> NiddMessage {
    NiddMessage {
      account_name: "9999080353-00001".to_string(),
      device_ids: vec![DeviceID {
        id: "9998501090".to_string(),
        kind: "MDN".to_string(),
      }],
      maximum_delivery_time: 400,
      message: "SEVMTE8=".to_string(),
    }
  }

  fn message_of(bytes: usize) -> String {
    let data = vec![0u8; bytes];
    let mut buf = vec![0u8; Base64::encoded_len(&data)];
    Base64::encode(&data, &mut buf).unwrap().to_string()
  }

  #[test]
  fn the_documented_request_passes_and_serializes_as_verizon_expects() {
    let msg = documented_message();

    assert_eq!(msg.validate(), Ok(()));
    assert_eq!(
      serde_json::to_string(&msg).unwrap(),
      concat!(
        r#"{"accountName":"9999080353-00001","deviceIds":[{"id":"9998501090","kind":"MDN"}],"#,
        r#""maximumDeliveryTime":400,"message":"SEVMTE8="}"#
      )
    );
  }

  #[test]
  fn decoded_len_counts_bytes_not_digits() {
    assert_eq!(decoded_len(""), 0);
    assert_eq!(decoded_len("QQ=="), 1);
    assert_eq!(decoded_len("QUI="), 2);
    assert_eq!(decoded_len("QUJD"), 3);
    assert_eq!(decoded_len("SEVMTE8="), 5);
    assert_eq!(decoded_len("SEVMTE8"), 5);
  }

  #[test]
  fn a_message_over_the_limit_is_refused() {
    let mut msg = documented_message();

    msg.message = message_of(MAX_NIDD_BYTES);
    assert_eq!(msg.validate(), Ok(()));

    msg.message = message_of(MAX_NIDD_BYTES + 1);
    assert_eq!(
      msg.validate(),
      Err(NiddMessageError::TooLarge(MAX_NIDD_BYTES + 1))
    );
  }

  #[test]
  fn a_delivery_time_outside_the_range_is_refused() {
    let mut msg = documented_message();

    for secs in [2, 86_400, 2_592_000] {
      msg.maximum_delivery_time = secs;
      assert_eq!(msg.validate(), Ok(()));
    }
    for secs in [-1, 0, 1, 2_592_001] {
      msg.maximum_delivery_time = secs;
      assert_eq!(msg.validate(), Err(NiddMessageError::DeliveryTime(secs)));
    }
  }
}
