use dioxus::prelude::*;
pub mod api;

use strum::IntoEnumIterator;
#[cfg(feature = "api")]
use thingspace_sdk::models::{CallbackListener, Device, ServiceName};
// use thingspace_sdk::models::{CallbackListener, LoginResponse, Session};
use dioxus::logger::tracing::{error, info, warn};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug)]
struct LocalSession {
  listeners: Signal<Vec<CallbackListener>>,
  devices: Signal<Vec<Device>>,
}

// #[cfg(feature = "api")]
// #[derive(Clone, Copy, Debug)]
// struct LocalSession {
//   access_token: Signal<LoginResponse>,
//   session_token: Signal<Session>,
//   listeners: Signal<Vec<CallbackListener>>,
// }

#[cfg(feature = "api")]
#[component]
pub fn App() -> Element {
  use_context_provider(|| LocalSession {
    listeners: Signal::new(vec![CallbackListener::default()]),
    devices: Signal::new(vec![Device::default()]),
  });

  rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    document::Link { rel: "icon", href: asset!("/assets/images/favicon.ico") }
    CreateListenerModal {}
    Index {}
  }
}

// #[cfg(feature = "api")]
// #[component]
// pub fn App() -> Element {
//   use_context_provider(|| LocalSession {
//     access_token: Signal::new(LoginResponse {
//       access_token: "".to_string(),
//       expires_in: 0,
//       scope: "".to_string(),
//       token_type: "Bearer".to_string(),
//     }),
//     session_token: Signal::new(Session {
//       session_token: "".to_string(),
//       expires_in: 0,
//     }),
//     listeners: Signal::new(vec![CallbackListener {
//       account_name: Some(String::with_capacity(16)),
//       ..Default::default()
//     }]),
//   });

//   rsx! {
//     document::Link { rel: "stylesheet", href: MAIN_CSS }
//     document::Link { rel: "icon", href: asset!("/assets/images/favicon.ico") }

//     Index {}
//   }
// }

#[cfg(feature = "browser")]
#[component]
pub fn Index() -> Element {
  use_resource(move || async move { api::login().await });
  rsx! {
    div { class: "flex flex-col justify-around h-full w-full lg:w-9/10",
      div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
        div { class: "card-body",
          h2 { class: "card-title", "VZW Access Token" }
          code { class: "whitespace-pre-line break-all", "" }
          div { class: "justify-center card-actions",
            button { class: "btn btn-primary", "Show Info" }
          }
        }
      }
    }
  }
}

#[cfg(feature = "api")]
#[component]
pub fn Index() -> Element {
  // let _ = use_resource(move || async move {
  //   api::callback_list().await;
  // });
  let _ = use_resource(move || async move {
    api::listener_list().await;
    api::device_list().await;
  });

  rsx! {
    div { class: "flex flex-col justify-around h-full w-full lg:w-9/10",
      div {
        h2 { class: "text-2xl", "Registered Callback Listeners" }
        div { class: "overflow-x-auto rounded-box border border-base-content/30",
          table { class: "table",
            thead {
              tr {
                th { "Account name" }
                th { "Service name" }
                th { "URL" }
                th {}
              }
            }
            tbody {
              for l in use_context::<crate::LocalSession>().listeners.iter() {
                tr {
                  if let Some(aname) = &l.account_name {
                    td { "{aname}" }
                  } else {
                    td { "" }
                  }
                  td { "{l.service_name}" }
                  td { "{l.url}" }
                  td {
                    {
                        let sname = l.service_name.clone();
                        rsx! {
                          button {
                            class: "btn btn-sm btn-error btn-outline",
                            onclick: move |_| {
                                let sname = sname.to_owned();
                                async move {
                                    api::delete_listener(&sname).await;
                                }
                            },
                            "x"
                          }
                        }
                    }
                  }
                }
              }
              tr {
                td {}
                td {}
                td {}
                td {
                  button {
                    class: "btn btn-sm btn-outline",
                    onclick: move |_| {
                        document::eval(
                            r#"document.getElementById("create_listener_modal").showModal();"#,
                        );
                    },
                    "+"
                  }
                }
              }
            }
          }
        }
      }

      div {
        h2 { class: "text-2xl", "Devices" }
        div { class: "overflow-x-auto rounded-box border border-base-content/30",
          table { class: "table",
            thead {
              tr {
                th { "IMEI" }
                th { "ICCID" }
                th { "Connected" }
                th { "Last connection date" }
                th { "Data usage" }
                th { "SMS usage" }
              }
            }
            tbody {
              for device in use_context::<crate::LocalSession>().devices.iter() {
                tr {
                  for did in device.device_ids.iter() {
                    if did.kind == "imei" {
                      td { "{did.id}" }
                    }
                  }
                  for did in device.device_ids.iter() {
                    if did.kind == "iccId" {
                      td { "{did.id}" }
                    }
                  }
                  td { "{device.connected}" }
                  td { "{device.last_connection_date}" }
                  td { "" }
                  td { "" }
                }
              }
            }
          }
        }
      }
    }
  }
}

// let access = use_context::<crate::LocalSession>().access_token;
// let session = use_context::<crate::LocalSession>().session_token;
// div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
//   div { class: "card-body",
//     h2 { class: "card-title", "VZW Access Token" }
//     code { class: "whitespace-pre-line break-all", "{access}" }
//   }
// }
// div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
//   div { class: "card-body",
//     h2 { class: "card-title", "VZW Session Token" }
//     code { class: "whitespace-pre-line break-all", "{session}" }
//   }
// }

#[component]
pub fn CreateListenerModal() -> Element {
  // let mut cbl = use_signal(CallbackListener::default);
  // let mut values = use_signal(std::collections::HashMap::new);

  rsx! {
    dialog { class: "modal", id: "create_listener_modal",
      div { class: "modal-box relative max-w-xs md:max-w-sm",
        form { class: "absolute end-2 top-2", method: "dialog",
          // onsubmit: move |evt: FormEvent| async move {
          //     evt.metadata.borrow_mut().prevent_default = false;
          // },
          button { class: "btn btn-sm btn-circle btn-ghost", "X" }
        }
        div { class: "text-center text-xl font-medium", "Register Callback Listener" }
        form {
          onsubmit: move |evt: FormEvent| async move {
              evt.prevent_default();
              let cbl = CallbackListener {
                  service_name: evt.values()["service_name"].as_value(),
                  url: evt.values()["url"].as_value(),
                  username: if evt.values()["username"].as_value().is_empty() {
                      None
                  } else {
                      Some(evt.values()["username"].as_value())
                  },
                  password: if evt.values()["password"].as_value().is_empty() {
                      None
                  } else {
                      Some(evt.values()["password"].as_value())
                  },
                  account_name: None,
              };
              api::create_listener(&cbl).await;
          },
          fieldset { class: "fieldset mt-5",
            legend { class: "fieldset-legend", "Service Name" }
            select {
              class: "select",
              name: "service_name",
              required: true,
              // option { disabled: true, selected: true, "Service Name" }
              for s in ServiceName::iter() {
                option { "{s}" }
              }
            }
          }
          fieldset { class: "fieldset mt-5",
            legend { class: "fieldset-legend", "URL" }
            label { class: "input w-full focus:outline-0",
              input {
                class: "grow focus:outline-0",
                name: "url",
                placeholder: "URL",
                r#type: "url",
                required: true,
              }
            }
          }
          fieldset { class: "fieldset mt-5",
            legend { class: "fieldset-legend", "Username" }
            label { class: "input w-full focus:outline-0",
              input {
                class: "grow focus:outline-0",
                name: "username",
                placeholder: "Username",
                r#type: "text",
                required: false,
              }
            }
            span { class: "label", "Optional" }
          }
          fieldset { class: "fieldset mt-5",
            legend { class: "fieldset-legend", "Password" }
            label { class: "input w-full focus:outline-0",
              input {
                class: "grow focus:outline-0",
                name: "password",
                placeholder: "Password",
                r#type: "password",
                required: false,
              }
            }
            span { class: "label", "Optional" }
          }
          div { class: "mt-5 flex items-center justify-end gap-3",
            button { class: "btn btn-primary", r#type: "submit",
              // onsubmit: move |evt: FormEvent| async move {
              //     evt.prevent_default();
              // },
              "Register"
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
