use dioxus::prelude::*;

#[component]
pub fn PlaybackControls() -> Element {
    let mut playing = use_signal(|| false);

    rsx! {
        div {class: "transport",
            button {class: "transport-btn", "⏮"}
            button {
                class: "play-btn",
                onclick: move |_| playing.toggle(),
                if playing() {"II"} else {"▶"}
            }
            button {class: "transport-btn", "⏭"}
            span {class: "muted", "00:00 / 00:00"}
        }
    }
}
