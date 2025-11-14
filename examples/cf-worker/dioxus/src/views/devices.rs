use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn DevicesView() -> Element {
  rsx! {
    DevicesTable {}
  }
}

#[component]
fn DevicesTable() -> Element {
  rsx! {
    div { class: "mt-5",
      h2 { class: "text-2xl", "Devices" }
      div { class: "overflow-x-auto rounded-box border border-base-content/30",
        table { class: "table",
          thead {
            tr {
              th { "IMEI" }
              th { "ICCID" }
              th { "Connected" }
              th { "Last connection date" }
            }
          }
          tbody {
            for (imei , device) in use_context::<crate::LocalSession>().devices.read().iter() {
              tr {
                td {
                  Link {
                    to: Route::DeviceView {
                        id: imei.clone(),
                    },
                    class: "link link-hover link-primary",
                    "{imei}"
                  }
                }
                td {
                  for did in device.device_ids.iter() {
                    if did.kind == "iccId" {
                      "{did.id}"
                    }
                  }
                }
                td { "{device.connected}" }
                td { "{device.last_connection_date}" }
              }
            }
          }
        }
      }
    }
  }
}
