use dioxus::prelude::*;

#[component]
pub fn Topbar() -> Element {
    rsx! {
        header{class: "topbar",
            div {class: "brand",
                span{class: "brand-icon", "F"}
                span{"FLOW"}
                span{class : "muted", "STUDIO"}
            }
            div {class: "project-name", "Unititled Project"}
            div {class: "top-actions",
                button{class: "secondary-btn", "Save Project"}
                button{class: "export-btn", "Export Video"}
            }
        }
    }
}
