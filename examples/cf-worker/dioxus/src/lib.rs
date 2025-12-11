use dioxus::prelude::*;
use std::collections::HashMap;
use thingspace_sdk::models::{CallbackListener, Device};
use views::{CallbackView, DeviceView, DevicesView, Navbar};

use crate::{
  api::{device_list, listener_list},
  websocket::{Message, WebsocketError},
};

use dioxus::logger::tracing::{error, info};
use futures::{SinkExt, stream::StreamExt};
// use gloo_net::websocket::{Message, futures::WebSocket};

pub mod api;
mod encoding;
mod views;
mod websocket;

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

// static WEBSOCKET: GlobalSignal<gloo_net::websocket::futures::WebSocket> = Signal::global(|| {
//   gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:8787/connect").unwrap()
// });

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

  // use_resource(move || async move {
  //   device_list().await;
  //   listener_list().await;
  // });

  // let ws: gloo_net::websocket::futures::WebSocket =
  //   gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:8787/connect").unwrap();
  // let (mut write, mut read) = ws.split();

  let mut socket = websocket::use_websocket(connect_to_websocket);

  // spawn(async move {
  //   write
  //     .send(Message::Text(String::from("test")))
  //     .await
  //     .unwrap();
  //   write
  //     .send(Message::Text(String::from("test 2")))
  //     .await
  //     .unwrap();
  // });

  spawn(async move {
    // match socket.send("test".to_string()).await {
    //   Ok(_) => todo!(),
    //   Err(e) => error!("{e:?}"),
    // }
    socket
      .send_raw(Message::Binary("test 2".into()))
      .await
      .unwrap();
  });

  //   dioxus_core::spawn_forever(async move {
  //     while let Some(msg) = read.next().await {
  //       info!("{:?}", msg)
  //     }
  //     info!("WebSocket Closed")
  //   });

  use_future(move || async move {
    while let Ok(msg) = socket.recv_raw().await {
      info!("{:?}", msg);
    }
    // while let msg = socket.recv_raw().await {
    //   match msg {
    //     Ok(m) => info!("{m:?}"),
    //     Err(e) => info!("{:?}", e),
    //   }
    // }
    info!("WebSocket Closed")
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

async fn connect_to_websocket() -> Result<websocket::Websocket> {
  let ws: gloo_net::websocket::futures::WebSocket =
    gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:8787/connect").unwrap();
  let (write, read) = ws.split();
  let split: websocket::WebsysSocket = websocket::WebsysSocket {
    sender: write.into(),
    receiver: read.into(),
  };
  //  ws.split().into();
  //   sender: Mutex<SplitSink<WsWebsocket, WsMessage>>,
  //   receiver: Mutex<SplitStream<WsWebsocket>>,
  // }

  Ok(
    websocket::Websocket::<String, String, encoding::JsonEncoding> {
      protocol: None,
      _in: std::marker::PhantomData,
      web: Some(split),
      response: None,
    },
  )

  // spawn(async move {
  //   write
  //     .send(Message::Text(String::from("test")))
  //     .await
  //     .unwrap();
  //   write
  //     .send(Message::Text(String::from("test 2")))
  //     .await
  //     .unwrap();
  // });

  // dioxus_core::spawn_forever(async move {
  //   while let Some(msg) = read.next().await {
  //     info!("{:?}", msg)
  //   }
  //   info!("WebSocket Closed")
  // });
}

// pub fn use_websocket<
//   In: 'static,
//   Out: 'static,
//   E: Into<gloo_net::websocket::WebSocketError> + 'static,
//   F: Future<Output = Result<gloo_net::websocket::futures::WebSocket, E>> + 'static,
// >(
//   mut connect_to_websocket: impl FnMut() -> F + 'static,
// ) -> gloo_net::websocket::futures::WebSocket {
//   let mut waker = use_waker();
//   let mut status = use_signal(|| gloo_net::websocket::State::Connecting);
//   let status_read = use_hook(|| ReadSignal::new(status));

//   let connection = use_resource(move || {
//     let connection = connect_to_websocket().map_err(|e| e.into());
//     async move {
//       let connection =
//         gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:8787/connect").unwrap();
//       // Update the status based on the result of the connection attempt
//       match connection.as_ref() {
//         Ok(_) => status.set(gloo_net::websocket::State::Open),
//         Err(_) => status.set(gloo_net::websocket::State::Closed),
//       }

//       // Wake up the `.recv()` calls waiting for the connection to be established
//       waker.wake(());

//       connection
//     }
//   });
//   gloo_net::websocket::futures::WebSocket {
//     connection,
//     sink_waker: waker,
//     status,
//     status_read,
//   }
// }
