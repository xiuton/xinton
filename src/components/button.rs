use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = props.class.as_deref().unwrap_or("px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition");
    rsx! {
        button {
            class: "{class}",
            onclick: move |evt| {
                if let Some(cb) = &props.onclick {
                    cb.call(evt);
                }
            },
            {props.children}
        }
    }
} 