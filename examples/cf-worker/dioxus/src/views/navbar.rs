use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    header { class: "navbar shadow-sm bg-base-200",
      nav { class: "navbar justify-evenly",
        Link { to: Route::Index {}, class: "link link-hover", "Home" }
        Link { to: Route::CallbackView {}, class: "link link-hover", "Callback Listeners" }
      }
    }
    div { class: "flex flex-col justify-around h-full w-full lg:w-9/10", Outlet::<Route> {} }
  }
}
