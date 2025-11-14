use dioxus::prelude::*;
use std::collections::HashMap;
use thingspace_sdk::models::{CallbackListener, Device};
use views::{CallbackView, DeviceView, DevicesView, Navbar};

use crate::api::{device_list, listener_list};

pub mod api;
mod views;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
  #[layout(Navbar)]
  #[route("/")]
  Index {},
  #[route("/callback_listeners")]
  CallbackView {},
  #[route("/device/:id")]
  DeviceView { id: String },
}

#[derive(Clone, Copy, Debug)]
struct LocalSession {
  listeners: Signal<Vec<CallbackListener>>,
  devices: Signal<HashMap<String, Device>>,
}

#[component]
pub fn App() -> Element {
  use_context_provider(|| LocalSession {
    listeners: Signal::new(vec![CallbackListener::default()]),
    devices: Signal::new(HashMap::<String, Device>::new()),
  });

  use_resource(move || async move {
    device_list().await;
    listener_list().await;
  });

  rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    document::Link { rel: "icon", href: asset!("/assets/images/favicon.ico") }
    Router::<Route> {}
  }
}

#[component]
pub fn Index() -> Element {
  rsx! {
    DevicesView {}
  }
}
