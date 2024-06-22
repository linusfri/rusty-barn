use axum::Json;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Faq {
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
                answer: "Follow a recipe and bake at 350 degrees, even if the recipe doesn't say so.".to_string()
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

pub async fn get_mock_data() -> Json<Vec<Faq>> {
    let faqs = Faq::mock_many();

    Json(faqs)
}