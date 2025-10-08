use crate::cache;
use thingspace_sdk::api::devices_list;
use thingspace_sdk::models::AccountDeviceListRequest;
use worker::{Request, Response, RouteContext, console_error};

pub async fn list_devices(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
  console_error_panic_hook::set_once();

  let atoken = cache::access_token(&ctx).await?;
  let stoken = cache::session_token(&ctx).await?;
  let aname = ctx.var("ACCOUNT_NAME")?;

  let adl = AccountDeviceListRequest {
    account_name: Some(aname.to_string()),
    device_id: None,
    filter: None,
    current_state: None,
    earliest: None,
    latest: None,
    service_plan: None,
    max_number_of_devices: None,
    largest_device_id_seen: None,
  };

  let vz_req = devices_list(&atoken, &stoken, &adl).await;

  match vz_req {
    Ok(resp) => Ok(resp),
    Err(e) => {
      console_error!("{e}");
      Response::error(e.to_string(), 500)
    }
  }
}
