use dioxus::prelude::*;
pub mod api;

use thingspace_sdk::models::{LoginResponse, Session};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Clone, Copy, Debug)]
struct LocalSession {
  access_token: Signal<LoginResponse>,
  session_token: Signal<Session>,
}

#[component]
pub fn App() -> Element {
  use_context_provider(|| LocalSession {
    access_token: Signal::new(LoginResponse {
      access_token: "".to_string(),
      expires_in: 0,
      scope: "".to_string(),
      token_type: "Bearer".to_string(),
    }),
    session_token: Signal::new(Session {
      session_token: "".to_string(),
      expires_in: 0,
    }),
  });

  rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    document::Link { rel: "icon", href: asset!("/assets/images/favicon.ico") }

    Index {}
  }
}

#[component]
pub fn Index() -> Element {
  // let secrets = Secrets {
  //   public_key: "bc9247a8-82e9-4aab-9a42-0711d91fc123".to_string(),
  //   private_key: "898cee3c-53a3-454c-9be8-2781130b3f0a".to_string(),
  //   username: "JESENGI".to_string(),
  //   password: "5#wS6gruyHzwS5".to_string(),
  //   account_name: "0742644905-00001".to_string(),
  // };
  //   let resp = use_resource(move || {
  //   let value = secrets.to_owned();
  //   async move { get_access_token(value).await }
  // });

  let acc_resp = use_resource(move || async move { api::fetch_access_token().await });
  let ses_resp = use_resource(move || async move { api::fetch_session_token().await });

  let mut access = use_context::<crate::LocalSession>().access_token;
  let mut session = use_context::<crate::LocalSession>().session_token;
  rsx! {
    div { class: "flex flex-col justify-around h-full w-full lg:w-9/10",
      div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
        div { class: "card-body",
          h2 { class: "card-title", "VZW Access Token" }
          code { class: "whitespace-pre-line break-all", "{access}" }
          div { class: "justify-center card-actions",
            button {
              class: "btn btn-primary",
              onclick: move |_| {
                  access
                      .set(
                          match &*acc_resp.read() {
                              Some(Ok(acc_resp)) => acc_resp.to_owned(),
                              Some(Err(err)) => {
                                  LoginResponse {
                                      access_token: format!("{:#?}", err),
                                      expires_in: 0,
                                      scope: "".to_string(),
                                      token_type: "Bearer".to_string(),
                                  }
                              }
                              None => LoginResponse::default(),
                          },
                      );
              },
              "Show Info"
            }
          }
        }
      }
      div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
        div { class: "card-body",
          h2 { class: "card-title", "VZW Session Token" }
          code { class: "whitespace-pre-line break-all", "{session}" }
          div { class: "justify-center card-actions",
            button {
              class: "btn btn-primary",
              onclick: move |_| {
                  session
                      .set(
                          match &*ses_resp.read() {
                              Some(Ok(ses_resp)) => ses_resp.to_owned(),
                              Some(Err(err)) => {
                                  Session {
                                      session_token: format!("{:#?}", err),
                                      expires_in: 0,
                                  }
                              }
                              None => Session::default(),
                          },
                      );
              },
              "Show Info"
            }
          }
        }
      }
    }
  }
}

// #[component]
// pub fn Index() -> Element {
//   let mut factor1 = use_signal(|| 1i32);
//   let mut factor2 = use_signal(|| 1i32);

//   let answer = {
//     let multiplication =
//       use_resource(move || async move { api::multiply(factor1(), factor2()).await });
//     let mut answer = use_signal(|| "?".to_string());
//     use_effect(move || {
//       answer.set(match &*multiplication.read() {
//         Some(Ok(resp)) => format!("{}", resp.product),
//         Some(Err(err)) => err.to_string(),
//         None => "= ?".to_string(),
//       });
//     });
//     answer
//   };

//   rsx! {
//     div { class: "min-w-6/10",
//       // Top row
//       div { class: "flex flex-row justify-around",
//         button {
//           class: "btn btn-soft",
//           onclick: move |_| {
//               factor1 += 1;
//           },
//           "+"
//         }
//         div {}
//         div {}
//         button {
//           class: "btn btn-soft",
//           onclick: move |_| {
//               factor2 += 1;
//           },
//           "+"
//         }
//       }

//       div { class: "flex flex-row justify-around",
//         // Middle row
//         div { "{factor1}" }
//         div { dangerous_inner_html: "&times;" }
//         div { "{factor2}" }
//         div { "=" }
//         div { "{answer}" }
//       }

//       // Bottom row

//       div { class: "flex flex-row justify-around",
//         button {
//           class: "btn btn-soft",
//           onclick: move |_| {
//               factor1 -= 1;
//           },
//           "-"
//         }
//         div {}
//         div {}
//         button {
//           class: "btn btn-soft",
//           onclick: move |_| {
//               factor2 -= 1;
//           },
//           "-"
//         }
//       }
//     }
//   }
// }
