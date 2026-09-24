use crate::models::Error;
use worker::{Fetch, Request, Response, console_error};

mod access;
pub use access::get_access_token;
pub use access::get_session_token;

mod registered_callback_listeners;
pub use registered_callback_listeners::deregister_callback_listener;
pub use registered_callback_listeners::list_callback_listeners;
pub use registered_callback_listeners::register_callback_listener;

mod devices;
pub use devices::devices_list;
pub use devices::send_nidd;

/// Sends `request`, answering [`Error::Api`] for an HTTP error status and any other response as
/// it came, for the caller to read.
async fn fetch(request: Request) -> Result<Response, Error> {
  match Fetch::Request(request).send().await {
    Ok(mut response) => {
      let status = response.status_code();
      if (400..600).contains(&status) {
        // An unreadable body still leaves the status, which is what a caller branches on.
        let body = response.bytes().await.unwrap_or_default();
        return Err(Error::api(status, &body));
      }
      Ok(response)
    }
    Err(e) => {
      console_error!("{:?}", e);
      Err(Error::Worker(e))
    }
  }
}
