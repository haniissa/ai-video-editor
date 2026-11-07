use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use dioxus_web::WebFileExt;

#[component]
pub fn MediaLibrary(on_video_selected: EventHandler<(String, String)>) -> Element {
    let mut videos = use_signal(Vec::<(String, String)>::new);
    let mut selected_video = use_signal(|| None::<String>);

    rsx! {
        aside {class: "panel media-panel",
                div {class: "panel-heading",
                    h3 {"Media Library"}
                    label {
                        class: "small-btn",
                        "+ Import"
                        input{
                            r#type: "file",
                            accept: "video/*",
                            multiple: true,
                            style: "display:none;",
                            onchange: move |event| {
                               #[cfg(target_arch="wasm32")]
                               {
                                   for file_data in event.files(){
                                       let filename = file_data.name();

                                       if let Some(file) = file_data.get_web_file(){
                                           if let Ok(url) = web_sys::Url::create_object_url_with_blob(&file){
                                               videos.write().push((filename.clone(), url.clone()));
                                               selected_video.set(Some(filename.clone()));
                                               on_video_selected.call((filename, url));
                                           }
                                       }
                                   }
                               }
                            }
                        }
                    }
                }
                div {class: "search-box", "⌕  Search media..."}

                for (filename, url) in videos(){
                    button {
                        class: if selected_video() == Some(filename.clone()){
                            "media-card selected"
                        }else{
                            "media-card"
                        },

                        onclick: move |_|{
                            selected_video.set(Some(filename.clone()));
                            on_video_selected.call((filename.clone(), url.clone()));
                        },
                        div {
                            class: "media-thumb",
                            "▶"
                        }
                        div {
                            strong {"{filename}"}
                            p {"MP4 · Video clip"}
                        }
                    }
                }

                if videos().is_empty(){
                    div {
                        class: "empty-hint",
                        "Your imported media will appear here."
                    }
                }
            }

    }
}
