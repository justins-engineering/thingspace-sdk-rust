use worker::{Context, Env, Request, Response, Result, Router, event};

#[cfg(feature = "browser")]
mod browser;

#[cfg(feature = "api")]
mod api;

mod cache;

#[cfg(all(feature = "browser", not(feature = "api")))]
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  Router::new()
    .post_async("/browser/access_token", access_token_browser)
    .run(req, env)
    .await
}

#[cfg(all(feature = "api", not(feature = "browser")))]
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  Router::new()
    .get_async("/api/callback_listener", api::list_listeners)
    .post_async("/api/callback_listener", api::create_listeners)
    .delete_async("/api/callback_listener/:name", api::delete_listeners)
    .post_async("/api/device", api::list_devices)
    .post_async("/api/send_nidd", api::send_nidd_msg)
    .or_else_any_method_async("/vzw", log_request)
    .run(req, env)
    .await
}

pub async fn log_request(
  mut req: Request,
  _ctx: worker::RouteContext<()>,
) -> worker::Result<Response> {
  worker::console_log!("{:#?}", req.headers());
  let body = req.text().await;

  match body {
    Ok(b) => worker::console_log!("{b}"),
    Err(e) => worker::console_error!("{e}"),
  }

  Response::empty()
}

// #[event(fetch)]
// async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
//   Router::new()
//     .post_async("/api/access_token", access_token_api)
//     .post_async("/api/session_token", session_token)
//     .run(req, env)
//     .await
// }

#[cfg(all(feature = "browser", feature = "api"))]
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  Router::new()
    .get_async("/api/callbacks", api::list_callbacks)
    .run(req, env)
    .await
}

// #[event(fetch)]
// async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
//   Router::new()
//     .post_async("/browser/access_token", browser::access_token)
//     .post_async("/api/access_token", api::access_token)
//     .post_async("/browser/session_token", browser::session_token)
//     .post_async("/api/session_token", api::session_token)
//     .run(req, env)
//     .await
// }
