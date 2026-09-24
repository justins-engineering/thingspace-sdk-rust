use crate::models::Error;

mod access;
pub use access::get_access_token;
pub use access::get_session_token;

mod devices;
pub use devices::devices_list;
pub use devices::send_nidd;

mod registered_callback_listeners;
pub use registered_callback_listeners::deregister_callback_listener;
pub use registered_callback_listeners::list_callback_listeners;
pub use registered_callback_listeners::register_callback_listener;

/// Sends `request`, answering [`Error::Api`] for an HTTP error status and any other response as
/// it came, for the caller to read.
async fn send(request: reqwest::RequestBuilder) -> Result<reqwest::Response, Error> {
  match request.send().await {
    Ok(response) => {
      let status = response.status().as_u16();
      if (400..600).contains(&status) {
        // An unreadable body still leaves the status, which is what a caller branches on.
        let body = response.bytes().await.unwrap_or_default();
        return Err(Error::api(status, &body));
      }
      Ok(response)
    }
    Err(e) => {
      println!("{e:?}");
      Err(Error::Reqwest(e))
    }
  }
}
