use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::AppRoute;
use crate::components::button::Button;

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    rsx! {
        h1 { class: "text-4xl font-extrabold text-blue-700 mb-4", "🏠 Home Page" }
        p { class: "text-lg text-gray-600 mb-8 text-center", "欢迎来到 Dioxus + TailwindCSS 示例项目！" }
        Button {
            class: "mb-4 bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-md text-white",
            onclick: move |_| { nav.push(AppRoute::AboutPage {}); },
            "跳转到 About"
        }
        a { href: "/game", "进入符号冒险探索游戏" }
    }
} 