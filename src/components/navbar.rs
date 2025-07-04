use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::AppRoute;
use super::button::Button;

#[component]
pub fn Navbar() -> Element {
    let nav = use_navigator();
    rsx! {
        nav { class: "w-full flex items-center justify-between p-2 bg-white/80 rounded-xl mt-4 shadow-md backdrop-blur sticky top-0 z-10 border border-black",
            div { class: "font-bold text-xl text-blue-700 flex items-center gap-2 select-none",
                span { "🦀" }
                span { class: "text-sm", "Dioxus Demo" }
            }
            div { class: "flex gap-2 overflow-x-auto",
                Button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 rounded-md whitespace-nowrap",
                    onclick: move |_| { nav.push(AppRoute::HomePage {}); },
                    "Home"
                }
                Button {
                    class: "bg-green-500 hover:bg-green-600 text-white px-2 py-1 rounded-md whitespace-nowrap",
                    onclick: move |_| { nav.push(AppRoute::AboutPage {}); },
                    "About"
                }
                Button {
                    class: "bg-yellow-500 hover:bg-yellow-600 text-white px-2 py-1 rounded-md whitespace-nowrap",
                    onclick: move |_| { nav.push("/game"); },
                    "Game"
                }
                Button {
                    class: "bg-red-500 hover:bg-red-600 text-white px-2 py-1 rounded-md whitespace-nowrap",
                    onclick: move |_| { nav.push("/not-found"); },
                    "Not Found"
                }
            }
        }
    }
} 