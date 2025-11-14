use crate::cache;
use thingspace_sdk::api::send_nidd;
use thingspace_sdk::models::NiddMessage;
use worker::{Request, Response, RouteContext, console_error};

pub async fn send_nidd_msg(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  let ctype = req.headers().get("Content-Type");

  let Ok(ctype) = ctype else {
    return Response::error("Missing 'Content-Type' header", 400);
  };
  let Some(ctype) = ctype else {
    return Response::error("Bad 'Content-Type' header", 400);
  };

  if ctype == "application/json" {
    let Ok(mut msg) = req.json::<NiddMessage>().await else {
      return Response::error("Request missing 'NiddMessage'", 400);
    };

    let atoken = cache::access_token(&ctx).await?;
    let stoken = cache::session_token(&ctx).await?;
    let aname = ctx.var("ACCOUNT_NAME")?;

    msg.account_name = aname.to_string();

    let vz_req = send_nidd(&atoken, &stoken, &mut msg).await;

    match vz_req {
      Ok(resp) => Ok(resp),
      Err(e) => {
        console_error!("{e}");
        Response::error(e.to_string(), 500)
      }
    }
  } else {
    Response::error("Wrong 'Content-Type' header", 400)
  }
}
