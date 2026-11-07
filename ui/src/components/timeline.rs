use dioxus::prelude::*;

fn format_time(seconds: f64) -> String {
    let total_seconds = seconds.round() as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

#[component]
pub fn Timeline(
    duration: f64,
    playhead_seconds: f64,
    on_playhead_change: EventHandler<f64>,
) -> Element {
    let mut selected_tool = use_signal(|| "Select".to_string());
    let mut clip_selected = use_signal(|| false);

    //Protect the timeline from invalid or missing video duration.
    let has_duration = duration.is_finite() && duration > 0.0;
    let safe_duration = if has_duration { duration } else { 0.0 };

    //Aim for approximately six ruler intervals
    let tick_step = if has_duration {
        ((safe_duration / 6.0).ceil() / 5.0).ceil() * 5.0
    } else {
        5.0
    };

    //Round the visible timeline scale up to a complete interval.
    let timeline_scale = if has_duration {
        (safe_duration / tick_step).ceil() * tick_step
    } else {
        30.0
    };

    let tick_count = (timeline_scale / tick_step) as usize;

    //Calculate how much of the timeline the video occupies.
    let clip_width = (safe_duration / timeline_scale) * 100.0;
    let playhead = playhead_seconds.clamp(0.0, safe_duration);

    let playhead_percent = if has_duration {
        (playhead / timeline_scale) * 100.0
    } else {
        0.0
    };

    rsx! {
        section{class: "timeline-panel",
            div{class: "timeline-heading",
                h3{"Timeline"}
                div {class: "tool-group",
                    for tool in  ["Select", "Cut", "Trim"]{
                        button {
                            class: if selected_tool() == tool {
                                "tool-btn active"
                            }else{
                                "tool-btn"
                            },
                            onclick: move |_| selected_tool.set(tool.to_string()),
                            "{tool}"
                        }
                    }
                }
            }
            //Dynamic timer ruler
            div {class: "ruler",
                for index in  0..=tick_count{
                    {
                        let time = index as f64 * tick_step;
                        rsx!{
                            span {
                                key: "{index}",
                                "{format_time(time)}"
                            }
                        }
                    }
                }
                div {
                    class: "playhead-marker",
                    style: "left: {playhead_percent:.2}%;"
                }
            }
            div{
                class: "playhead-controls",
                label {"Playhead: {format_time(playhead)}"}
                input{
                    r#type: "range",
                    min: "0",
                    max: "{safe_duration}",
                    step: "0.2",
                    value: "{playhead}",
                    oninput: move |event| {
                        if let Ok(seconds) = event.value().parse::<f64>(){
                            if seconds.is_finite(){
                                on_playhead_change.call(
                                    seconds.clamp(0.0, safe_duration)
                                );
                            }
                        }
                    }
                }
            }
            //Video track
            div {class: "track",
                div {class: "track-label", "V1"}
                div {class: "track-content",
                    if has_duration {
                        div {
                            class: if clip_selected(){
                                "clip video-clip selected"
                            }else{
                                "clip video-clip"
                            },
                            style: "width: {clip_width:.2}%;",
                            onclick: move |_| clip_selected.set(true),
                            strong {"Video clip"}
                            span {
                                "{format_time(0.0)} - {format_time(safe_duration)}"
                            }
                        }
                    }
                }
            }
            //Audio track
            div {class: "track",
                div{class: "track-label", "A1"}
                div{class: "track-content",
                    div {class: "clip audio-clip",
                        strong{"Audio track"}
                    }
                }
            }
            div {class: "timeline-footer",
                if has_duration{
                    "Video duration: {duration:.2} seconds"
                }else{
                    "Import a video to see its timeline."
                }
            }
        }
    }
}
