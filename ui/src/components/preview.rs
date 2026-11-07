use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[component]
pub fn Preview(
    video: Option<(String, String)>,
    on_duration_loaded: EventHandler<f64>,
    seek_to: ReadOnlySignal<f64>,
) -> Element {
    use_effect(move || {
        let target_seconds = seek_to();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id("preview-video") {
                        if let Ok(video_element) = element.dyn_into::<web_sys::HtmlVideoElement>() {
                            let duration = video_element.duration();

                            if duration.is_finite() {
                                let target = target_seconds.clamp(0.0, duration);

                                if (video_element.current_time() - target).abs() > 0.05 {
                                    video_element.set_current_time(target);
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    rsx! {
        div { class: "viewer-heading",
            span { "PREVIEW" }
            span { class: "muted", "16:9 · 1080p" }
        }

        div { class: "preview-area",
            div { class: "preview-frame",

                if let Some((filename, url)) = video {
                    video {
                        class: "preview-video",
                        id:"preview-video",
                        src: "{url}",
                        controls: true,
                        autoplay: false,
                        "aria-label": "{filename}",

                        onloadedmetadata: move |event| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(web_event) = event.downcast::<web_sys::Event>() {
                                    if let Some(target) = web_event.target() {
                                        if let Ok(video_element) =
                                            target.dyn_into::<web_sys::HtmlVideoElement>()
                                        {
                                            let duration = video_element.duration();
                                            if duration.is_finite() {
                                                on_duration_loaded.call(duration);
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                } else {
                    div { class: "preview-content",
                        div { class: "play-symbol", "▶" }
                        h2 { "Your story starts here." }
                        p { "Import a video to begin editing." }
                    }
                }

                span { class: "preview-time", "Video preview" }
            }
        }
    }
}
