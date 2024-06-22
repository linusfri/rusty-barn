pub mod image_proc;
pub mod about;

use std::collections::HashMap;
use lazy_static::lazy_static;
use tera::{Tera, Context};
use axum::{
    extract::{DefaultBodyLimit, Query},
    response::Html, routing::{get, post},
    Router
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use crate::config::Config;


// Load tera templates to make them available globally.
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("./src/public/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[derive(Clone)]
pub struct AppState {
    info: String
}

impl AppState {
    pub fn new(info: String) -> Self {
        Self {
            info
        }
    }
}

pub async fn serve(config: Config, state: Option<AppState>) {
    // Create a new router with a TraceLayer and State.
    let app = base_router()
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets_vite", ServeDir::new("./src/public/dist/assets"))
        .nest_service("/assets", ServeDir::new("./src/public/assets"))
        .layer(DefaultBodyLimit::max(256 * 1024 * 1024))
        .with_state(state.unwrap_or(
            AppState::new("Default".to_string())
        ));

    // Subscribe to tracing events from TraceLayer
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind(config.address_and_port).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn base_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about::get_mock_data))
        .route("/deepfry", post(image_proc::deepfry))
}

async fn index(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let mut context = Context::new();

    context.insert("params", &params);

    TEMPLATES.render("index.html", &context)
        .map(|s| Html(s))
        .unwrap()
}