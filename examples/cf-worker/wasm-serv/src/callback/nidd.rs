use thingspace_sdk::models::NiddCallback;
use worker::{Request, Response, RouteContext, console_error, console_log};

pub async fn receive_nidd_msg(
  mut req: Request,
  _ctx: RouteContext<()>,
) -> worker::Result<Response> {
  let ctype = req.headers().get("Content-Type");

  let Ok(ctype) = ctype else {
    return Response::error("Missing 'Content-Type' header", 400);
  };
  let Some(ctype) = ctype else {
    return Response::error("Bad 'Content-Type' header", 400);
  };

  if ctype == "application/json" {
    let Ok(body) = req.text().await else {
      console_error!("NIDD callback body unreadable");
      return Response::empty();
    };

    // The body carries the listener's password, the device's message and its identifiers, and
    // serde's error text can quote any of them: log the request id and status, or where it failed.
    match serde_json::from_str::<NiddCallback>(&body) {
      Ok(callback) => console_log!(
        "NIDD callback {} {:?}",
        callback.request_id,
        callback.status
      ),
      Err(e) => console_error!(
        "NIDD callback unparsed: {:?} at column {}",
        e.classify(),
        e.column()
      ),
    }
  } else {
    return Response::error("'Content-Type' must be 'application/json'", 400);
  }

  Response::empty()
}
