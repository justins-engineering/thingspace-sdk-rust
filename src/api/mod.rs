mod request_helpers;

#[cfg(feature = "wasm")]
mod web;
#[cfg(feature = "wasm")]
pub use web::get_access_token;
#[cfg(feature = "wasm")]
pub use web::get_session_token;

#[cfg(feature = "worker")]
mod worker;
#[cfg(feature = "worker")]
pub use worker::get_access_token;
#[cfg(feature = "worker")]
pub use worker::get_session_token;

#[cfg(feature = "ureq")]
mod linux;
#[cfg(feature = "ureq")]
/// Functions for use with "Device Management" API endpoints, primarily `/devices/actions/list`
pub use linux::devices_list;
#[cfg(feature = "ureq")]
pub use linux::get_access_token;
#[cfg(feature = "ureq")]
pub use linux::get_session_token;

#[cfg(feature = "ureq")]
/// Functions for use with "Registered Callbacks Listeners" API endpoints
pub use linux::deregister_callback_listener;
#[cfg(feature = "ureq")]
pub use linux::list_callback_listeners;
#[cfg(feature = "ureq")]
pub use linux::register_callback_listener;
