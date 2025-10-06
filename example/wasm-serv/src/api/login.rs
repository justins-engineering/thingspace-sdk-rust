use serde::{Deserialize, Serialize};
use worker::{Request, Response, RouteContext, Secret, console_error};

use thingspace_sdk::api::{get_access_token, get_session_token};
use thingspace_sdk::models::SessionRequestBody;

#[derive(Deserialize, Serialize, Debug)]
struct Access {
  access_token: String,
}

pub async fn access_token(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let public_key = ctx.var("PUBLIC_KEY")?;
  let private_key = ctx.var("PRIVATE_KEY")?;

  let vz_req = get_access_token(&public_key.to_string(), &private_key.to_string()).await;

  match vz_req {
    Ok(resp) => Ok(resp),
    Err(e) => {
      console_error!("{:?}", e);
      Response::error(e.to_string(), 500)
    }
  }
}

pub async fn session_token(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let ctype = req.headers().get("Content-Type");

  let Ok(ctype) = ctype else {
    return Response::error("Missing 'Content-Type' header", 400);
  };
  let Some(ctype) = ctype else {
    return Response::error("Bad 'Content-Type' header", 400);
  };

  if ctype == "application/json" {
    let Ok(token) = req.json::<Access>().await else {
      return Response::error("Request missing 'access_token'", 400);
    };

    let env = ctx.env;
    let username: Secret = env.var("USERNAME")?;
    let password = env.var("PASSWORD")?;

    let cred = SessionRequestBody {
      username: username.to_string(),
      password: password.to_string(),
    };

    if token.access_token.is_empty() {
      return Response::error("Request missing 'access_token'", 400);
    }

    let vz_req = get_session_token(&cred, &token.access_token).await;

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
