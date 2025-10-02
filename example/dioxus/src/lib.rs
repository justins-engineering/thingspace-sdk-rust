use dioxus::prelude::*;
pub mod api;

#[cfg(feature = "api")]
use thingspace_sdk::models::{LoginResponse, Session};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug)]
struct LocalSession {
  access_token: Signal<LoginResponse>,
  session_token: Signal<Session>,
}

#[cfg(feature = "api")]
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
  let _ = use_resource(move || async move { api::login().await });

  let access = use_context::<crate::LocalSession>().access_token;
  let session = use_context::<crate::LocalSession>().session_token;
  rsx! {
    div { class: "flex flex-col justify-around h-full w-full lg:w-9/10",
      div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
        div { class: "card-body",
          h2 { class: "card-title", "VZW Access Token" }
          code { class: "whitespace-pre-line break-all", "{access}" }
        }
      }
      div { class: "card w-full bg-neutral text-neutral-content card-lg shadow-sm",
        div { class: "card-body",
          h2 { class: "card-title", "VZW Session Token" }
          code { class: "whitespace-pre-line break-all", "{session}" }
        }
      }
    }
  }
}
