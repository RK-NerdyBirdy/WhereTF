mod api;
#[cfg(feature = "bundled-backend")]
mod backend;
mod components;

use components::{header::Header, results_screen::ResultsScreen, search_screen::SearchScreen};
use dioxus::prelude::*;
use dioxus_desktop::WindowCloseBehaviour;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    #[cfg(feature = "bundled-backend")]
    backend::ensure_backend_blocking_spawn();

    let css = format!(
        "<style>{}{}{}{}{}</style>",
        include_str!("style.css"),
        include_str!("components/header.css"),
        include_str!("components/search_bar.css"),
        include_str!("components/search_screen.css"),
        include_str!("components/results_screen.css"),
    );

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop! {
            dioxus_desktop::Config::new()
                .with_custom_head(css)
                .with_menu(None)
                .with_close_behaviour(WindowCloseBehaviour::WindowHides)
                .with_window(
                    dioxus_desktop::WindowBuilder::new()
                        .with_title("whereTF Lite")
                        .with_theme(Some(dioxus_desktop::tao::window::Theme::Dark))
                        .with_inner_size(dioxus_desktop::LogicalSize::new(900.0, 650.0)),
                )
        })
        .launch(app);
}

fn app() -> Element {
    let search_results = use_signal(Vec::<api::SearchResult>::new);
    let query_text = use_signal(String::new);
    let search_query = use_signal(String::new);
    let search_time = use_signal(|| 0.0);
    let uploading = use_signal(|| false);
    let upload_trigger = use_signal(|| 0i32);
    let file_paths = use_signal(HashMap::<String, String>::new);
    let backend_ready = use_signal(|| false);
    let backend_error = use_signal(|| Option::<String>::None);

    use_effect(move || {
        let mut backend_ready = backend_ready.clone();
        let mut backend_error = backend_error.clone();
        spawn(async move {
            for _ in 0..5 {
                if let Ok(response) = reqwest::get("http://127.0.0.1:8000/health").await {
                    if response.status().is_success() {
                        backend_ready.set(true);
                        return;
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            backend_error.set(Some(
                "Backend not running. Start it with: docker compose up --build -d".to_string(),
            ));
        });
    });

    use_effect(move || {
        let query = search_query.read().clone();
        if query.trim().is_empty() || !backend_ready() {
            return;
        }
        let mut search_results = search_results.clone();
        let mut query_text = query_text.clone();
        let mut search_time = search_time.clone();
        spawn(async move {
            let started = Instant::now();
            if let Ok(response) = api::search(&query, 10).await {
                query_text.set(query);
                search_time.set(started.elapsed().as_secs_f64());
                search_results.set(response.results);
            }
        });
    });

    use_effect(move || {
        let count = upload_trigger();
        if count <= 0 || uploading() {
            return;
        }
        let mut uploading = uploading.clone();
        let mut file_paths = file_paths.clone();
        spawn(async move {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                uploading.set(true);
                let path_str = path.to_string_lossy().to_string();
                let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                file_paths.write().insert(filename, path_str.clone());
                let _ = api::upload_file(&path_str).await;
                uploading.set(false);
            }
        });
    });

    let on_open_file = use_callback(move |path: String| {
        let resolved = file_paths.read().get(&path).cloned().unwrap_or(path);
        let _ = open::that(resolved);
    });

    rsx! {
        div { id: "main",
            div { class: "app-shell",
                Header { view_label: "/ LITE MVP" }
                if !backend_ready() {
                    div { class: "backend-loading", style: "padding: 48px; font-family: var(--font-mono); color: var(--text-secondary);",
                        if let Some(error) = backend_error.read().as_ref() {
                            "{error}"
                        } else {
                            "Starting lite backend..."
                        }
                    }
                } else if search_results.read().is_empty() {
                    SearchScreen {
                        on_search: search_query.clone(),
                        uploading: uploading(),
                        upload_trigger: upload_trigger.clone(),
                    }
                } else {
                    ResultsScreen {
                        query: query_text.read().clone(),
                        search_time: search_time(),
                        results: search_results.read().clone(),
                        on_open_file: on_open_file.clone(),
                        on_search: search_query.clone(),
                    }
                }
            }
        }
    }
}
