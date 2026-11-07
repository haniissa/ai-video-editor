use dioxus::prelude::*;

mod components;
use components::{Inspector, MediaLibrary, PlaybackControls, Preview, Statusbar, Timeline, Topbar};
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let mut selected_video = use_signal(|| None::<(String, String)>);
    let mut video_duration = use_signal(|| 0.0_f64);
    let mut playhead_seconds = use_signal(|| 0.0_f64);

    rsx! {
        div { class: "editor",
            Topbar {}

            main { class: "workspace",
                MediaLibrary {
                    on_video_selected: move |video: (String, String)| {
                        let is_new_video = selected_video() != Some(video.clone());

                        selected_video.set(Some(video));
                        if is_new_video{
                            video_duration.set(0.0);
                            playhead_seconds.set(0.0);
                        }
                    },
                }

                section { class: "center-panel",
                    Preview {
                        video: selected_video(),
                        on_duration_loaded: move |duration: f64| {
                            video_duration.set(duration);
                        },
                        seek_to: playhead_seconds,
                    }
                    PlaybackControls {}
                    Timeline {
                        duration: video_duration(),
                        playhead_seconds: playhead_seconds(),
                        on_playhead_change: move |seconds: f64|{
                            playhead_seconds.set(seconds);
                        }
                    }
                }

                Inspector {}
            }

            Statusbar {}
        }
    }
}
