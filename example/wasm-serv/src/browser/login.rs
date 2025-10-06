use serde::{Deserialize, Serialize};
use worker::{
  Request, Response, ResponseBuilder, RouteContext, Secret, console_error, console_warn,
};

use thingspace_sdk::api::{get_access_token, get_session_token};
use thingspace_sdk::models::{LoginResponse, Session, SessionRequestBody};

#[derive(Deserialize, Serialize, Debug)]
struct Access {
  access_token: String,
}

pub async fn access_token(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let auth = req.headers().get("Cookie");
  console_warn!("{auth:?}");
  match auth {
    Ok(op) => {
      if let Some(auth) = op {
        let ac = serde_urlencoded::from_str::<Access>(&auth)?;
        console_warn!("{ac:?}");
        if !ac.access_token.is_empty() {
          return Response::empty();
        }
        console_warn!("Authorization cookie present, but empty");
      } else {
        console_warn!("No Authorization cookie present");
      };
    }
    Err(e) => {
      console_error!("{:?}", e);
    }
  };

  let public_key = ctx.var("PUBLIC_KEY")?;
  let private_key = ctx.var("PRIVATE_KEY")?;

  let vz_req = get_access_token(&public_key.to_string(), &private_key.to_string()).await;

  match vz_req {
    Ok(mut resp) => match resp.json::<LoginResponse>().await {
      Ok(login) => {
        console_warn!("{login}");

        let mut cookie = String::with_capacity(128);

        cookie.push_str("access_token=");
        cookie.push_str(&login.access_token);
        cookie.push_str("; Secure; Domain=localhost; HttpOnly; Max-Age=");
        cookie.push_str(&login.expires_in.to_string());

        let headers = worker::Headers::new();
        headers.append("Set-Cookie", &cookie)?;

        Ok(
          ResponseBuilder::new()
            .with_headers(headers)
            .with_status(204)
            .empty(),
        )
      }
      Err(_) => Response::error("Unexpected response", 502),
    },
    Err(e) => {
      console_error!("{:?}", e);
      Response::error(e.to_string(), 500)
    }
  }
}

pub async fn session_token(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let head: Result<Option<String>, worker::Error> = req.headers().get("Cookie");
  match head {
    Ok(op) => {
      match op {
        Some(auth) => {
          let ac: Access = serde_urlencoded::from_str::<Access>(&auth)?;
          console_warn!("{}", ac.access_token);
          let env = ctx.env;
          let username: Secret = env.var("USERNAME")?;
          let password = env.var("PASSWORD")?;

          let cred = SessionRequestBody {
            username: username.to_string(),
            password: password.to_string(),
          };

          let vz_req = get_session_token(&cred, &ac.access_token).await;

          match vz_req {
            // Ok(resp) => Ok(resp),
            Ok(mut resp) => match resp.json::<Session>().await {
              Ok(login) => {
                console_warn!("{login}");

                let mut cookie = String::with_capacity(128);

                cookie.push_str("bearer=");
                cookie.push_str(&login.session_token);
                cookie.push_str("; Secure; Domain=localhost; HttpOnly; Max-Age=");
                cookie.push_str(&login.expires_in.to_string());

                let headers = worker::Headers::new();
                headers.append("Set-Cookie", &cookie)?;

                Ok(
                  ResponseBuilder::new()
                    .with_headers(headers)
                    .with_status(204)
                    .empty(),
                )
              }
              Err(_) => Response::error("Unexpected response", 502),
            },
            Err(e) => {
              console_error!("{:?}", e);
              Response::error(e.to_string(), 500)
            }
          }
        }
        None => Response::error("", 400),
      }
    }
    Err(e) => {
      console_error!("{:?}", e);
      Response::error(e.to_string(), 500)
    }
  }
}
