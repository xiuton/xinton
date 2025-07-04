use dioxus_router::prelude::Outlet;
use dioxus::prelude::*;
use crate::router::AppRoute;
use super::navbar::Navbar;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "min-h-screen w-screen max-w-screen-sm mx-auto bg-gray-50",
            div { class: "min-h-screen bg-gradient-to-br from-red-50 to-yellow-100 flex flex-col items-center gap-4 px-4",
                Navbar {}
                main { class: "w-full bg-white rounded-xl shadow-lg p-4 flex flex-col items-center border border-black",
                    Outlet::<AppRoute> {}
                }
            }
        }
    }
}