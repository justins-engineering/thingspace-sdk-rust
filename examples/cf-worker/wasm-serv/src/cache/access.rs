use worker::{RouteContext, Secret, console_debug, console_error};

use thingspace_sdk::api::{get_access_token, get_session_token};
use thingspace_sdk::models::{LoginResponse, Session, SessionRequestBody};

pub async fn access_token(ctx: &RouteContext<()>) -> worker::Result<String> {
  let kv = ctx.kv("THINGSPACE")?;
  let access = kv.get("access_token").text().await?;

  match access {
    None => {
      console_debug!("No access_token cached");
      let public_key: Secret = ctx.var("PUBLIC_KEY")?;
      let private_key = ctx.var("PRIVATE_KEY")?;

      let vz_req: Result<worker::Response, thingspace_sdk::models::Error> =
        get_access_token(&public_key.to_string(), &private_key.to_string()).await;

      match vz_req {
        Ok(mut resp) => match resp.json::<LoginResponse>().await {
          Ok(login) => {
            console_debug!("{login}");

            kv.put("access_token", &login.access_token)?
              .expiration_ttl(login.expires_in.try_into().unwrap())
              .metadata(&login.scope)?
              .execute()
              .await?;

            Ok(login.access_token)
          }
          Err(e) => {
            console_error!("{e}");
            Err(e)
          }
        },
        Err(e) => {
          console_error!("{:?}", e);
          Err(worker::Error::RustError(e.to_string()))
        }
      }
    }
    Some(token) => {
      console_debug!("Cached access_token: {token}");
      Ok(token)
    }
  }
}

pub async fn session_token(ctx: &RouteContext<()>) -> worker::Result<String> {
  let kv = ctx.kv("THINGSPACE")?;
  let sesssion = kv.get("session_token").text().await?;

  match sesssion {
    None => {
      console_debug!("No session_token cached");
      let key = kv.get("access_token").text().await?;
      match key {
        Some(access_token) => {
          let username: Secret = ctx.var("USERNAME")?;
          let password = ctx.var("PASSWORD")?;

          let cred: SessionRequestBody = SessionRequestBody {
            username: username.to_string(),
            password: password.to_string(),
          };

          let vz_req = get_session_token(&cred, &access_token).await;

          match vz_req {
            Ok(mut resp) => match resp.json::<Session>().await {
              Ok(login) => {
                console_debug!("{login}");

                kv.put("session_token", &login.session_token)?
                  .expiration_ttl(login.expires_in.try_into().unwrap())
                  .execute()
                  .await?;

                Ok(login.session_token)
              }
              Err(e) => {
                console_error!("{e}");
                Err(e)
              }
            },
            Err(e) => {
              console_error!("{:?}", e);
              Err(worker::Error::RustError(e.to_string()))
            }
          }
        }
        None => Err(worker::Error::RustError(
          "Empty 'access_token' value!".to_string(),
        )),
      }
    }
    Some(token) => {
      console_debug!("Cached session_token: {token}");
      Ok(token)
    }
  }
}
