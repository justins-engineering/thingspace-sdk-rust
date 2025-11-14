use crate::api::{create_listener, delete_listener};
use dioxus::prelude::*;
use strum::IntoEnumIterator;
use thingspace_sdk::models::{CallbackListener, ServiceName};

#[component]
pub fn CallbackView() -> Element {
  rsx! {
    CallbackTable {}
    CreateListenerModal {}
  }
}

#[component]
fn CallbackTable() -> Element {
  rsx! {
    div { class: "mt-5",
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
                                  delete_listener(&sname).await;
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
  }
}

#[component]
fn CreateListenerModal() -> Element {
  rsx! {
    dialog { class: "modal", id: "create_listener_modal",
      div { class: "modal-box relative max-w-xs md:max-w-sm",
        form { class: "absolute end-2 top-2", method: "dialog",

          button { class: "btn btn-sm btn-circle btn-ghost", "X" }
        }
        div { class: "text-center text-xl font-medium", "Register Callback Listener" }
        form {
          onsubmit: move |evt: FormEvent| async move {
              evt.prevent_default();
              let mut cbl = CallbackListener::default();
              for (key, val) in evt.values() {
                  if let FormValue::Text(val) = val {
                      if key == "service_name" {
                          cbl.service_name = val;
                      } else if key == "url" {
                          cbl.url = val;
                      } else if key == "username" {
                          cbl.username = Some(val);
                      } else if key == "password" {
                          cbl.password = Some(val);
                      } else if key == "account_name" {
                          cbl.account_name = Some(val);
                      }
                  }
              }
              create_listener(&cbl).await;
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
