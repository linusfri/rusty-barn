use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::Arc, thread};
use lazy_static::lazy_static;
use tera::{Tera, Context};
use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage};
use axum::{
    extract::{DefaultBodyLimit, Multipart, Query}, response::{Html, Redirect}, routing::{get, post}, Form, Json, Router
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
        .route("/hej", post(post_fn))
        .route("/faq", get(get_mock_data))
        .route("/img", post(img_proc))
}

async fn index(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let mut context = Context::new();

    context.insert("params", &params);

    TEMPLATES.render("index.html", &context)
        .map(|s| Html(s))
        .unwrap()
}

async fn get_mock_data() -> Json<Vec<Faq>> {
    let faqs = Faq::mock_many();

    Json(faqs)
}

async fn img_proc(mut image_data: Multipart) -> Html<String> {
    let mut files: HashMap<String, DynamicImage> = HashMap::new();

    while let Some(field) = image_data.next_field().await.unwrap_or(None) {
        let file_name = String::from(field.file_name().unwrap());
        let byte_img = field.bytes().await;
        let bytes = match byte_img {
            Ok(image) => image,
            Err(_) => continue
                
        };
        let img = ImageReader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
    
        files.insert(String::from(file_name), img);
    };
    
    let image_html: String = files.iter().map(|(name, image)| {
        format!("<li>{}</li>", name)
    }).collect();

    let mut join_handles: Vec<thread::JoinHandle::<()>> = vec![];

    files.into_iter().for_each(|(name, file)| {
        let handle = thread::spawn(move || {
            let modified = file.adjust_contrast(50.0).huerotate(90);
    
            modified.save(format!("src/public/assets/{}.png", name)).unwrap();
        });

        join_handles.push(handle);
    });

    join_handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    Html(image_html)
}

async fn post_fn(Form(data): Form<PostForm>) -> Redirect {
    let mut context = Context::new();
    context.insert("data", &data);

    Redirect::to("/?form_submit=true")
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PostForm {
    name: String
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Faq {
    question: String,
    answer: String
}

impl Faq {
    pub fn mock_many() -> Vec<Self> {
        // A vec where each Faq has differnt question and answers
        vec![
            Faq {
                question: "What is the meaning of life?".to_string(),
                answer: "42".to_string()
            },
            Faq {
                question: "How do you make a cake?".to_string(),
                answer: "Follow a recipe and bake at 350 degrees.".to_string()
            },
            Faq {
                question: "What is Rust programming language?".to_string(),
                answer: "A systems programming language focused on safety and performance.".to_string()
            },
            Faq {
                question: "How does gravity work?".to_string(),
                answer: "A force that attracts a body towards the center of the earth.".to_string()
            },
            Faq {
                question: "What is the speed of light?".to_string(),
                answer: "Approximately 299,792 kilometers per second.".to_string()
            },
            Faq {
                question: "What is the capital of France?".to_string(),
                answer: "Paris".to_string()
            },
            Faq {
                question: "Who wrote 'To Kill a Mockingbird'?".to_string(),
                answer: "Harper Lee".to_string()
            },
            Faq {
                question: "What is the powerhouse of the cell?".to_string(),
                answer: "Mitochondria".to_string()
            },
            Faq {
                question: "How many continents are there?".to_string(),
                answer: "Seven".to_string()
            }
        ]
    }
}   