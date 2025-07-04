use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::AppRoute;
use crate::components::button::Button;
use crate::config::AppConfig;

#[component]
pub fn About() -> Element {
    let nav = use_navigator();
    let config = AppConfig::get_default();
    
    rsx! {
        h1 { class: "text-4xl font-extrabold text-green-700 mb-4", "ℹ️ About Page" }
        p { class: "text-lg text-gray-600 mb-8 text-center", "本项目基于 Dioxus + TailwindCSS，适合快速构建现代 Web 应用。" }
        
        div { class: "mb-6",
            Button {
                class: "mb-4 bg-green-600 hover:bg-green-700 px-4 py-2 rounded-md text-white",
                onclick: move |_| { nav.push(AppRoute::HomePage {}); },
                "返回 Home"
            }
        }
        
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h2 { class: "text-2xl font-bold text-gray-800 mb-4", "📋 配置信息" }
            p { class: "text-gray-600 mb-2", {"应用名称: ".to_string() + &config.app.name} }
            p { class: "text-gray-600 mb-2", {"应用版本: ".to_string() + &config.app.version} }
            p { class: "text-gray-600 mb-2", {"应用描述: ".to_string() + &config.app.description} }
            p { class: "text-gray-600 mb-2", {"服务器主机: ".to_string() + &config.server.host} }
            p { class: "text-gray-600 mb-2", {"服务器端口: ".to_string() + &config.server.port.to_string()} }
            p { class: "text-gray-600 mb-2", {"UI 主题: ".to_string() + &config.ui.theme} }
            p { class: "text-gray-600 mb-2", {"UI 语言: ".to_string() + &config.ui.language} }
            p { class: "text-gray-600 mb-2", {"UI 时区: ".to_string() + &config.ui.timezone} }
        }
        
        div { class: "bg-blue-50 p-6 rounded-lg",
            h3 { class: "text-lg font-semibold text-blue-800 mb-2", "💡 配置使用说明" }
            p { class: "text-blue-700 mb-2", "通过实现 Display trait，可以直接在 RSX 中显示整个配置对象：" }
            p { class: "bg-gray-100 p-3 rounded text-sm", {"p { \"UI 相关配置: ".to_string() + &config.ui.to_string() + "\" }"} }
            p { class: "text-blue-700 mt-2", "这样会显示: 主题: light, 语言: zh-CN, 时区: Asia/Shanghai" }
        }
    }
} 