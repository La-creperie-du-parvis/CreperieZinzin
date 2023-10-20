use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct AskPrompt {
    model: String,
    prompt: String,
    stream: bool,
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
    let prompt: AskPrompt = AskPrompt {
        model: "llama2:13b".to_owned(),
        prompt: "Please provide a Array of 8 ingredients for a dinner recipe. Example:  [\"ingredient1\", \"ingredient2\", \"ingredient3\", \"ingredient4\", \"ingredient5\", \"ingredient6\", \"ingredient7\", \"ingredient8\"]".to_owned(),
        stream : false // Ollama's API endpoints such as /api/generate now support returning data in one single response. Set the stream parameter to false in the API request:
    };

    let prompt_to_json: String = serde_json::to_string(&prompt).unwrap();

    let client: reqwest::Client = reqwest::Client::new();

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post("http://localhost:11434/api/generate")
        .body(prompt_to_json)
        .send()
        .await;

    if let Ok(response) = res {
        let body: String = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read response body".to_string());

        let body_parsed_to_json: Value = serde_json::from_str(&body).unwrap();
        let get_response_from_body: Option<&str> = body_parsed_to_json["response"].as_str();

        return HttpResponse::Ok().body(String::from(get_response_from_body.unwrap()));
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
