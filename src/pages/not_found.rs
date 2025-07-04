use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::AppRoute;
use crate::components::button::Button;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let nav = use_navigator();
    rsx! {
        h1 { class: "text-4xl font-extrabold text-red-600 mb-4", "🚫 404 Not Found" }
        p { class: "text-lg text-gray-600 mb-4 text-center", "页面不存在: " span { class: "font-mono text-red-500", {segments.join("/")} } }
        Button {
            class: "bg-red-600 hover:bg-red-700 px-4 py-2 rounded-md text-white",
            onclick: move |_| { nav.push(AppRoute::HomePage {}); },
            "返回 Home"
        }
    }
} 