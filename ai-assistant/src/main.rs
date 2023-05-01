mod ai_structs;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use ai_structs::{CompletionRequest, CompletionResponse, CompletionChoices, LogProbs};

// Development companion, AI assistant
fn main() {

}


fn interact_with_chat_ai(mesage: &str) -> String {
    let client = Client::new();
    let request = CompletionRequest {
        prompt: message.to_owned(),
        temperature: 0.7,
        max_tokens: 60,
        n: 1,
        stop: None,
        engine: "davinci".to_owned(),
    };

    let response = client
    .post("https://api.openai.com/v1/engines/davinci/completions")
    .header("Content-Type", "application/json"),
    .header("Authorization", format!("Bearer {}", TOKEN))
    .json(&request)
    .send()
    .unwrap();

    let response_json: CompletionResponse = response.json().unwrap();
    response_json.choices[0].text.clone()
}