use dioxus::prelude::*;

#[component]
pub fn Inspector() -> Element {
    rsx! {
        aside { class: "panel inspector-panel",
            div {class: "panel-heading",
                h3{"Inspector"}
            }
            div{class: "inspector-empty",
                div {class: "inspector-icon", "◈"}
                strong {"Nothing selected"}
                p {"Select a clip on the timeline to inspect its properties."}
            }
            div {class: "properties",
                h4{"PROJECT SETTINGS"}

                div{class: "property-row",
                    span {"Resolution"}
                    strong{"1920 x 1080"}
                }
                div {class: "property-row",
                    span{"Frame rate"}
                    strong{"30 FPS"}
                }
                div {class: "property-row",
                    span {"Aspect ratio"}
                    strong{"16:9"}
                }
            }
        }
    }
}
