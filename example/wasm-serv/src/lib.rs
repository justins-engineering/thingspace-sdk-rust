use serde::{Deserialize, Serialize};
use worker::{
  Context, Env, Request, Response, Result, RouteContext, Router, Secret, console_error, event,
};

use thingspace_sdk::api::{get_access_token, get_session_token};
use thingspace_sdk::models::{Secrets, SessionRequestBody};

#[derive(Deserialize, Serialize, Debug)]
struct Access {
  access_token: String,
}

// async fn read_request_body(mut req: Request) -> std::result::Result<String, worker::Error> {
//   let ctype = req.headers().get("Content-Type").unwrap().unwrap();
//   match ctype.as_str() {
//     "application/json" => {
//       req.json::<Access>().await
//       // match body {}
//       // format!("{:?}", req.json::<Access>().await.unwrap())
//     }
//     "text/html" => req.text().await.unwrap(),
//     "multipart/form-data" => format!("{:?}", req.form_data().await.unwrap()),
//     _ => String::from("a file"),
//   }
// }

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  Router::new()
    // .get_async("/api/multiply", multiply)
    .post_async("/api/access_token", access_token)
    .post_async("/api/session_token", session_token)
    .run(req, env)
    .await
}

// pub async fn multiply(req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
//   console_error_panic_hook::set_once();

//   let uri = req.url()?;

//   let Some(query) = uri.query() else {
//     return Response::error("expected query parameters", 400);
//   };
//   let Ok(request) = serde_urlencoded::from_str::<MultiplyRequest>(query) else {
//     return Response::error("BAD REQUEST", 400);
//   };

//   match request.factor1.checked_mul(request.factor2) {
//     Some(product) => Ok(worker::ResponseBuilder::new().from_json(&MultiplyResponse { product })?),
//     None => Response::error("BAD REQUEST", 400),
//   }
// }

pub async fn access_token(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  // let uri = req.url()?;

  // let secrets = read_secrets_from_file().expect("Failed to read from secrets.toml");

  let env = _ctx.env;
  let public_key = env.var("PUBLIC_KEY")?;
  let private_key = env.var("PRIVATE_KEY")?;
  let username: Secret = env.var("USERNAME")?;
  let password = env.var("PASSWORD")?;
  let account_name = env.var("ACCOUNT_NAME")?;

  let secrets = Secrets {
    public_key: public_key.to_string(),
    private_key: private_key.to_string(),
    username: username.to_string(),
    password: password.to_string(),
    account_name: account_name.to_string(),
  };

  // let body = req.json::<Secrets>().await?;

  // let Some(sec) = body else {
  //   return Response::error("expected query parameters", 400);
  // };

  // Response::error("", 400)

  let vz_req = get_access_token(secrets).await;

  match vz_req {
    // Ok(resp) => Ok(worker::ResponseBuilder::new().from_json(&resp)?),
    Ok(resp) => Ok(resp),
    Err(e) => {
      console_error!("{:?}", e);
      Response::error(e.to_string(), 500)
    }
  }
}

pub async fn session_token(mut req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  // let access_token = read_request_body(req).await;

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

    let env = _ctx.env;
    let username: Secret = env.var("USERNAME")?;
    let password = env.var("PASSWORD")?;

    let cred = SessionRequestBody {
      username: username.to_string(),
      password: password.to_string(),
    };

    if token.access_token.is_empty() {
      return Response::error("Request missing 'access_token'", 400);
    }

    let vz_req = get_session_token(cred, &token.access_token).await;

    match vz_req {
      // Ok(resp) => Ok(worker::ResponseBuilder::new().from_json(&resp)?),
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
