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
/// Functions for use with "Registered Callbacks Listeners" API endpoints
pub use worker::deregister_callback_listener;
#[cfg(feature = "worker")]
/// Functions for use with "Device Management" API endpoints, primarily `/devices/actions/list`
pub use worker::devices_list;
#[cfg(feature = "worker")]
pub use worker::get_access_token;
#[cfg(feature = "worker")]
pub use worker::get_session_token;
#[cfg(feature = "worker")]
pub use worker::list_callback_listeners;
#[cfg(feature = "worker")]
pub use worker::register_callback_listener;
#[cfg(feature = "worker")]
pub use worker::send_nidd;

#[cfg(feature = "reqwest")]
mod native;
#[cfg(feature = "reqwest")]
/// Functions for use with "Registered Callbacks Listeners" API endpoints
pub use native::deregister_callback_listener;
#[cfg(feature = "reqwest")]
/// Functions for use with "Device Management" API endpoints, primarily `/devices/actions/list`
pub use native::devices_list;
#[cfg(feature = "reqwest")]
pub use native::get_access_token;
#[cfg(feature = "reqwest")]
pub use native::get_session_token;
#[cfg(feature = "reqwest")]
pub use native::list_callback_listeners;
#[cfg(feature = "reqwest")]
pub use native::register_callback_listener;
#[cfg(feature = "reqwest")]
pub use native::send_nidd;
