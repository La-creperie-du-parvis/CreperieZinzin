use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use reqwest;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AskPrompt {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct AnswerPrompt {
    response: String,
    ingredients: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[post("/chat")]
async fn chat() -> HttpResponse {
    let prompt = AskPrompt {
        model: "llama2:13b".to_owned(),
        prompt: "Please provide a Array of 8 ingredients for a dinner recipe. Example:  [\"ingredient1\", \"ingredient2\", \"ingredient3\", \"ingredient4\", \"ingredient5\", \"ingredient6\", \"ingredient7\", \"ingredient8\"]".to_owned(),
        stream : false // Ollama's API endpoints such as /api/generate now support returning data in one single response. Set the stream parameter to false in the API request:
    };

    let prompt_to_json = serde_json::to_string(&prompt).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .body(prompt_to_json)
        .send()
        .await;

    if let Ok(response) = res {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read response body".to_string());
        let responses: Vec<&str> = body.split('}').collect();
        let concatenated_responses = responses
            .iter()
            .map(|response| {
                let response = format!("{}{}", response, '}');
                let parsed: serde_json::Value =
                    serde_json::from_str(&response).unwrap_or_else(|_| serde_json::json!({}));
                let response_value = parsed["response"].as_str().unwrap_or("");
                response_value.to_string()
            })
            .collect::<Vec<String>>()
            .join("");

        let test_json = serde_json::to_string(&concatenated_responses).unwrap();
        println!("{:?}", test_json);
        return HttpResponse::Ok().body(concatenated_responses);
    }

    HttpResponse::InternalServerError().body("Error")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index).service(chat)
        // .route("/chat", web::post().to(chat))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
