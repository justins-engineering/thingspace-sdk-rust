use serde::{Deserialize, Serialize};
use std::{error, fmt};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialError {
  /// ThingSpace error description
  pub error_description: String,
  /// ThingSpace Error reason
  pub error: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThingSpaceError {
  /// ThingSpace error code
  #[serde(rename = "errorCode")]
  pub error_code: String,
  /// ThingSpace error message
  #[serde(rename = "errorMessage")]
  pub error_message: String,
}

#[derive(Debug)]
pub enum Error {
  #[cfg(any(feature = "wasm", feature = "worker"))]
  Js(web_sys::wasm_bindgen::JsValue),
  #[cfg(any(feature = "wasm", feature = "worker"))]
  SerdeURL(serde_urlencoded::ser::Error),
  #[cfg(any(feature = "wasm", feature = "worker"))]
  SerdeWasm(serde_wasm_bindgen::Error),
  #[cfg(feature = "worker")]
  Worker(worker::Error),
  Serde(serde_json::Error),
  Credential(CredentialError),
  ThingSpace(ThingSpaceError),
  UTF8(std::str::Utf8Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (module, e) = match self {
      #[cfg(any(feature = "wasm", feature = "worker"))]
      Error::Js(e) => ("JsValueError", format!("{e:?}")),
      #[cfg(any(feature = "wasm", feature = "worker"))]
      Error::SerdeURL(e) => ("SerdeUrlEncodeError", e.to_string()),
      #[cfg(any(feature = "wasm", feature = "worker"))]
      Error::SerdeWasm(e) => ("SerdeWasmBindgenError", e.to_string()),
      #[cfg(feature = "worker")]
      Error::Worker(e) => ("WorkerError", e.to_string()),
      Error::Serde(e) => ("SerdeError", e.to_string()),
      Error::Credential(e) => (
        "CredentialError",
        format!("\"{}\": \"{}\"", &e.error, &e.error_description),
      ),
      Error::ThingSpace(e) => (
        "ThingSpaceError",
        format!("\"{}\": \"{}\"", &e.error_code, &e.error_message),
      ),
      Error::UTF8(e) => ("Utf8Error", e.to_string()),
    };
    write!(f, "{{ \"{module}\": {{ {e} }} }}")
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    Some(match self {
      #[cfg(any(feature = "wasm", feature = "worker"))]
      Error::Js(_) => return None,
      #[cfg(any(feature = "wasm", feature = "worker"))]
      Error::SerdeURL(e) => e,
      #[cfg(any(feature = "wasm", feature = "worker"))]
      Error::SerdeWasm(e) => e,
      #[cfg(feature = "worker")]
      Error::Worker(e) => e,
      Error::Serde(e) => e,
      Error::Credential(_) => return None,
      Error::ThingSpace(_) => return None,
      Error::UTF8(e) => e,
    })
  }
}

#[cfg(any(feature = "wasm", feature = "worker"))]
impl From<web_sys::wasm_bindgen::JsValue> for Error {
  fn from(e: web_sys::wasm_bindgen::JsValue) -> Self {
    Error::Js(e)
  }
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Error::Serde(e)
  }
}

#[cfg(any(feature = "wasm", feature = "worker"))]
impl From<serde_wasm_bindgen::Error> for Error {
  fn from(e: serde_wasm_bindgen::Error) -> Self {
    Error::SerdeWasm(e)
  }
}

#[cfg(any(feature = "wasm", feature = "worker"))]
impl From<serde_urlencoded::ser::Error> for Error {
  fn from(e: serde_urlencoded::ser::Error) -> Self {
    Error::SerdeURL(e)
  }
}

impl From<std::str::Utf8Error> for Error {
  fn from(e: std::str::Utf8Error) -> Self {
    Error::UTF8(e)
  }
}

#[cfg(feature = "worker")]
impl From<worker::Error> for Error {
  fn from(e: worker::Error) -> Self {
    Error::Worker(e)
  }
}

impl From<CredentialError> for Error {
  fn from(e: CredentialError) -> Self {
    Error::Credential(e)
  }
}

impl From<ThingSpaceError> for Error {
  fn from(e: ThingSpaceError) -> Self {
    Error::ThingSpace(e)
  }
}

// #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
// pub struct GenericError {
//   /// The status code
//   #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
//   pub code: Option<i64>,
//   /// Debug information  This field is often not exposed to protect against leaking sensitive information.
//   #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
//   pub debug: Option<String>,
//   /// Further error details
//   #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
//   pub details: Option<serde_json::Value>,
//   /// The error ID  Useful when trying to identify various errors in application logic.
//   #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
//   pub id: Option<String>,
//   /// Error message  The error's message.
//   #[serde(rename = "message")]
//   pub message: String,
//   /// A human-readable reason for the error
//   #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
//   pub reason: Option<String>,
//   /// The request ID  The request ID is often exposed internally in order to trace errors across service architectures. This is often a UUID.
//   #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
//   pub request: Option<String>,
//   /// The status description
//   #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
//   pub status: Option<String>,
// }

// impl GenericError {
//   pub fn new(message: String) -> GenericError {
//     GenericError {
//       code: None,
//       debug: None,
//       details: None,
//       id: None,
//       message,
//       reason: None,
//       request: None,
//       status: None,
//     }
//   }
// }

// /// ErrorGeneric : The standard Ory JSON API error format.
// #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
// pub struct ErrorGeneric {
//   #[serde(rename = "error")]
//   pub error: Box<GenericError>,
// }

// impl ErrorGeneric {
//   /// The standard Ory JSON API error format.
//   pub fn new(error: GenericError) -> ErrorGeneric {
//     ErrorGeneric {
//       error: Box::new(error),
//     }
//   }
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// #[derive(Debug, Clone)]
// pub struct ResponseContent<T> {
//   pub status: u16,
//   pub content: String,
//   pub entity: Option<T>,
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// #[derive(Debug)]
// pub enum Error<T> {
//   Js(wasm_bindgen::JsValue),
//   Serde(serde_json::Error),
//   Io(std::io::Error),
//   ResponseError(ResponseContent<T>),
//   UTF8(std::str::Utf8Error),
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// impl<T> fmt::Display for Error<T> {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     let (module, e) = match self {
//       Error::Js(e) => ("wasm_bindgen", format!("{:?}", e)),
//       Error::Serde(e) => ("serde", e.to_string()),
//       Error::Io(e) => ("IO", e.to_string()),
//       Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
//       Error::UTF8(e) => ("UTF8", e.to_string()),
//     };
//     write!(f, "error in {}: {}", module, e)
//   }
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// impl<T: fmt::Debug> error::Error for Error<T> {
//   fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//     Some(match self {
//       Error::Js(_) => return None,
//       Error::Serde(e) => e,
//       Error::Io(e) => e,
//       Error::ResponseError(_) => return None,
//       Error::UTF8(e) => e,
//     })
//   }
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// impl<T> From<wasm_bindgen::JsValue> for Error<T> {
//   fn from(e: wasm_bindgen::JsValue) -> Self {
//     Error::Js(e)
//   }
// }

// impl<T> From<serde_json::Error> for Error<T> {
//   fn from(e: serde_json::Error) -> Self {
//     Error::Serde(e)
//   }
// }

// impl<T> From<std::io::Error> for Error<T> {
//   fn from(e: std::io::Error) -> Self {
//     Error::Io(e)
//   }
// }

// impl<T> From<std::str::Utf8Error> for Error<T> {
//   fn from(e: std::str::Utf8Error) -> Self {
//     Error::UTF8(e)
//   }
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// trait AddQuery {
//   fn add_query(&mut self, first_query: &mut bool, param: &str, value: &str);
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// impl AddQuery for String {
//   fn add_query(&mut self, first_query: &mut bool, param: &str, value: &str) {
//     if *first_query {
//       self.push('?');
//       *first_query = false;
//     } else {
//       self.push('&');
//     }
//     self.push_str(param);
//     self.push_str(value);
//   }
// }

// #[cfg(any(feature = "wasm", feature = "worker"))]
// #[allow(dead_code)]
// #[inline]
// fn add_query(first_query: &mut bool, uri: &mut String, param: &str, value: &str) {
//   if *first_query {
//     uri.push('?');
//     *first_query = false;
//   } else {
//     uri.push('&');
//   }
//   uri.push_str(param);
//   uri.push_str(value);
// }
