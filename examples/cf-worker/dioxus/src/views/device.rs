use crate::api::send_nidd;
use dioxus::prelude::*;
use thingspace_sdk::models::{DeviceID, NiddMessage};

#[component]
pub fn DeviceView(id: String) -> Element {
  // let Some(dev) = use_context::<crate::LocalSession>().devices.read().get(&id);
  rsx! {
    DeviceInfo { id: id.clone() }
    SendNiddModal { id }
  }
}

#[component]
fn DeviceInfo(id: String) -> Element {
  match use_context::<crate::LocalSession>().devices.read().get(&id) {
    Some(dev) => {
      rsx! {
        div { class: "my-5 flex flex-row justify-around items-center",
          if dev.connected {
            p { class: "text-2xl",
              span { class: "status status-lg status-success" }
              " Connected"
            }
          } else {
            p { class: "text-2xl",
              span { class: "status status-lg" }
              " Disconnected"
            }
          }
          button {
            class: "btn btn-outline",
            onclick: move |_| {
                document::eval(r#"document.getElementById("send_nidd_modal").showModal();"#);
            },
            "Send NIDD"
          }
        }

        div { class: "my-5",
          h2 { class: "text-2xl", "Device IDs" }
          div { class: "overflow-x-auto rounded-box border border-base-content/5",
            table { class: "table",
              thead {
                tr {
                  for did in dev.device_ids.iter() {
                    th { class: "border-r border-base-content/5",
                      "{did.kind.to_uppercase()}"
                    }
                  }
                }
              }
              tbody {
                tr {
                  for did in dev.device_ids.iter() {
                    td { class: "border-r border-base-content/5",
                      "{did.id}"
                    }
                  }
                }
              }
            }
          }
        }

        div { class: "my-5",
          h2 { class: "text-2xl", "Activity" }
          div { class: "overflow-x-auto rounded-box border border-base-content/5",
            table { class: "table",
              thead {
                tr {
                  // th { class: "border-r border-base-content/5", "Connected" }
                  th { class: "border-r border-base-content/5",
                    "Last connection date"
                  }
                  th { class: "border-r border-base-content/5", "Created at" }
                  th { class: "border-r border-base-content/5", "Last Activation By" }
                  th { class: "border-r border-base-content/5",
                    "Last Activation Date"
                  }
                }
              }
              tbody {
                tr {
                  // td { class: "border-r border-base-content/5", "{dev.connected}" }
                  td { class: "border-r border-base-content/5",
                    "{dev.last_connection_date}"
                  }
                  td { class: "border-r border-base-content/5", "{dev.created_at}" }
                  td { class: "border-r border-base-content/5",
                    "{dev.last_activation_by}"
                  }
                  td { class: "border-r border-base-content/5",
                    "{dev.last_activation_date}"
                  }
                }
              }
            }
          }
        }

        div { class: "my-5",
          h2 { class: "text-2xl", "Account Info" }
          div { class: "overflow-x-auto rounded-box border border-base-content/5",
            table { class: "table",
              thead {
                tr {
                  th { class: "border-r border-base-content/5", "Account Name" }
                  th { class: "border-r border-base-content/5", "Group Name" }
                }
              }
              tbody {
                tr {
                  td { class: "border-r border-base-content/5", "{dev.account_name}" }
                  td { class: "border-r border-base-content/5",
                    "{dev.group_names[0]}"
                  }
                }
              }
            }
          }
        }

        div { class: "my-5",
          h2 { class: "text-2xl", "Plan" }
          div { class: "overflow-x-auto rounded-box border border-base-content/5",
            table { class: "table",
              thead {
                tr {
                  th { class: "border-r border-base-content/5", "Carrier Name" }
                  th { class: "border-r border-base-content/5", "Service Plan" }
                  th { class: "border-r border-base-content/5", "State" }
                  th { class: "border-r border-base-content/5",
                    "Billing Cycle End Date"
                  }
                }
              }
              tbody {
                tr {
                  for cinf in dev.carrier_informations.iter() {
                    td { class: "border-r border-base-content/5",
                      "{cinf.carrier_name}"
                    }
                    td { class: "border-r border-base-content/5",
                      "{cinf.service_plan}"
                    }
                    td { class: "border-r border-base-content/5",
                      "{cinf.state}"
                    }
                    td { class: "border-r border-base-content/5",
                      "{dev.billing_cycle_end_date}"
                    }
                  }
                }
              }
            }
          }
        }

        div { class: "my-5",
          h2 { class: "text-2xl", "Extended Attributes" }
          ul { class: "list rounded-box border border-base-content/5",
            for ea in dev.extended_attributes.iter() {
              if let Some(val) = &ea.value && !val.is_empty() {
                li { class: "list-row grid-cols-2",
                  div { class: "col-span-1 border-r border-base-content/5",
                    "{ea.key}"
                  }
                  div { "{val}" }
                }
              }
            }
          }
        }
      }
    }
    None => rsx!(),
  }
}

#[component]
fn SendNiddModal(id: String) -> Element {
  rsx! {
    dialog { class: "modal", id: "send_nidd_modal",
      div { class: "modal-box relative max-w-xs md:max-w-sm",
        form { class: "absolute end-2 top-2", method: "dialog",
          button { class: "btn btn-sm btn-circle btn-ghost", "X" }
        }
        div { class: "text-center text-xl font-medium", "Send NIDD Message" }
        form {
          onsubmit: move |evt: FormEvent| {
              let imei = id.to_owned();
              async move {
                  evt.prevent_default();
                  let mut msg = NiddMessage::default();
                  for (key, val) in evt.values() {
                      if let FormValue::Text(val) = val {
                          if key == "maximum_delivery_time" {
                              msg.maximum_delivery_time = val
                                  .parse()
                                  .expect("Not a valid number");
                          } else if key == "message" {
                              msg.message = val;
                          }
                      }
                  }
                  msg.device_ids = vec![
                      DeviceID {
                          kind: "imei".to_string(),
                          id: imei,
                      },
                  ];
                  send_nidd(&msg).await;
              }
          },
          fieldset { class: "fieldset my-5",
            legend { class: "fieldset-legend", "Maximum Delivery Time" }
            input {
              class: "input validator w-full focus:outline-0",
              name: "maximum_delivery_time",
              r#type: "number",
              min: "2",
              max: "2592000",
              placeholder: "400",
              required: true,
            }
            p { class: "validator-hint",
              "The allowed range is between 2 secs and 2592000 secs (30 days)."
            }
          }
          fieldset { class: "fieldset my-5",
            legend { class: "fieldset-legend", "Message" }
            label { class: "input w-full focus:outline-0",
              input {
                class: "grow focus:outline-0",
                name: "message",
                placeholder: "Message",
                r#type: "text",
                required: true,
              }
            }
          }
          div { class: "my-5 flex items-center justify-end gap-3",
            button { class: "btn btn-primary", r#type: "submit",
              // onsubmit: move |evt: FormEvent| async move {
              //     evt.prevent_default();
              // },
              "Send"
            }
          }
        }
      }
      form { class: "modal-backdrop", method: "dialog",
        button { "close" }
      }
    }
  }
}
