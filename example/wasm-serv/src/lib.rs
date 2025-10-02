use worker::{Context, Env, Request, Response, Result, Router, event};

#[cfg(feature = "browser")]
mod browser;

#[cfg(feature = "api")]
mod api;

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
    .post_async("/api/access_token", access_token_api)
    .post_async("/api/session_token", session_token)
    .run(req, env)
    .await
}

#[cfg(all(feature = "browser", feature = "api"))]
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  Router::new()
    .post_async("/browser/access_token", browser::access_token)
    .post_async("/api/access_token", api::access_token)
    .post_async("/browser/session_token", browser::session_token)
    .post_async("/api/session_token", api::session_token)
    .run(req, env)
    .await
}
