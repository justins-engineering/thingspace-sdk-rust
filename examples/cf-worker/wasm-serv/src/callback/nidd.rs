use thingspace_sdk::models::NiddCallback;
use worker::{Request, Response, RouteContext, console_error};

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
    let body = req.json::<NiddCallback>().await;

    match body {
      Ok(b) => {
        worker::console_log!("{b:?}");
        // worker::console_log!("{:?}", b.nidd_response);
      }
      Err(e) => console_error!("{e}"),
    }
  } else {
    return Response::error("'Content-Type' must be 'application/json'", 400);
    // let body = req.text().await;
    // match body {
    //   Ok(b) => worker::console_log!("{b}"),
    //   Err(e) => worker::console_error!("{e}"),
    // }
  }

  Response::empty()
}
