use dioxus::prelude::*;
use crate::router::AppRoute;
use dioxus_router::prelude::Router;

pub fn app() -> Element {
    rsx! {
        Router::<AppRoute> {}
    }
}