use crate::cache;
use thingspace_sdk::api::{
  deregister_callback_listener, list_callback_listeners, register_callback_listener,
};
use thingspace_sdk::models::CallbackListener;
use worker::{Request, Response, RouteContext, console_error};

pub async fn list_listeners(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
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

pub async fn create_listeners(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let ctype: Result<Option<String>, worker::Error> = req.headers().get("Content-Type");

  let Ok(ctype) = ctype else {
    return Response::error("Missing 'Content-Type' header", 400);
  };
  let Some(ctype) = ctype else {
    return Response::error("Bad 'Content-Type' header", 400);
  };

  if ctype == "application/json" {
    let Ok(cbl) = req.json::<CallbackListener>().await else {
      return Response::error("Request missing 'CallbackListener'", 400);
    };

    if cbl.service_name.is_empty() {
      return Response::error("CallbackListener missing 'service_name'", 400);
    }

    if cbl.url.is_empty() {
      return Response::error("CallbackListener missing 'url'", 400);
    }

    let aname = ctx.var("ACCOUNT_NAME")?;
    let atoken = cache::access_token(&ctx).await?;
    let stoken = cache::session_token(&ctx).await?;

    let vz_req = register_callback_listener(&aname.to_string(), &atoken, &stoken, &cbl).await;

    match vz_req {
      Ok(resp) => Ok(resp),
      Err(e) => {
        console_error!("{:?}", e);
        Response::error(e.to_string(), 500)
      }
    }
  } else {
    Response::error("Wrong 'Content-Type' header", 400)
  }
}

pub async fn delete_listeners(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  if let Some(sname) = ctx.param("name") {
    let aname = ctx.var("ACCOUNT_NAME")?;
    let atoken = cache::access_token(&ctx).await?;
    let stoken = cache::session_token(&ctx).await?;

    let vz_req = deregister_callback_listener(&aname.to_string(), &atoken, &stoken, sname).await;

    match vz_req {
      Ok(resp) => Ok(resp),
      Err(e) => {
        console_error!("{:?}", e);
        Response::error(e.to_string(), 500)
      }
    }
  } else {
    Response::error("Missing 'service_name' parameter in url", 400)
  }
}
