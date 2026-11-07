use dioxus::prelude::*;

#[component]
pub fn Statusbar() -> Element {
    rsx! {
        footer { class: "statusbar",
            span { "● Ready" }
            span { "FLOW STUDIO · UI PROTOTYPE" }
        }
    }
}
