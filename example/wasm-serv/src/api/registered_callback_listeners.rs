use worker::{Request, Response, RouteContext, console_error};

use crate::cache;
use thingspace_sdk::api::list_callback_listeners;

pub async fn list_callbacks(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let aname = ctx.var("ACCOUNT_NAME")?;
  let atoken = cache::access_token(&ctx).await?;
  let stoken = cache::session_token(&ctx).await?;

  let vz_req = list_callback_listeners(&aname.to_string(), &atoken, &stoken).await;

  match vz_req {
    Ok(resp) => Ok(resp),
    Err(e) => {
      console_error!("{:?}", e);
      Response::error(e.to_string(), 500)
    }
  }
}
