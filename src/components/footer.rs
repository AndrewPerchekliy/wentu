use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    static DEFAULT_COVER: Asset = asset!("/assets/img/default-album-artwork.png", ImageAssetOptions::new().with_avif());
    rsx! {
        footer {
            div { id: "current_track_info",
                img { src: DEFAULT_COVER, id: "current_track_cover" }
                div { id: "current_track_credits",
                    p { id:"current_track_title", "Uknown Track" }
                    p { id:"current_track_artist", "Uknown Artist" }
                }
            }
            div {  }
            div { 
                button { class: "playback-buttons",
                    
                }
                button { class: "playback-buttons", 

                }
                button { class: "playback-buttons",

                }
            }
            div {  }
        }
    }
}
