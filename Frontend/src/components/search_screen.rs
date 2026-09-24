use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchScreenProps {
    pub on_search: Signal<String>,
    pub uploading: bool,
    pub upload_trigger: Signal<i32>,
}

#[component]
pub fn SearchScreen(props: SearchScreenProps) -> Element {
    rsx! {
        div { class: "search-screen",
            div { class: "search-heading", "Search your files." }
            super::search_bar::SearchBar {
                on_search: props.on_search.clone(),
                placeholder: "type to search".to_string(),
            }
            div { class: "search-stats",
                span { "LITE SEMANTIC SEARCH" }
                span { "\u{00B7}" }
                span {
                    class: "upload-link",
                    onclick: {
                        let mut ut = props.upload_trigger.clone();
                        move |_| { let v = *ut.peek(); ut.set(v + 1); }
                    },
                    if props.uploading { "uploading..." } else { "+ add file" }
                }
            }
        }
    }
}
