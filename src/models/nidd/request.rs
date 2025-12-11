use serde::{Deserialize, Serialize};

/// A struct containing a NIDD Message request id.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NiddRequest {
  /// requestId for async NIDD message
  pub request_id: String,
}

impl Default for NiddRequest {
  fn default() -> NiddRequest {
    NiddRequest {
      request_id: String::with_capacity(36),
    }
  }
}
