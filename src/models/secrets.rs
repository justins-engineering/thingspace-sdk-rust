use serde::{Deserialize, Serialize};

/// A struct containing a user's Verizon account secrets required for API use.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Secrets {
  pub public_key: String,
  pub private_key: String,
  pub username: String,
  pub password: String,
  pub account_name: String,
}
