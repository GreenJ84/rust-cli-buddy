pub mod gpt_structs;

use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};

use crate::gpt_structs::{CompletionRequest, CompletionResponse, Message};


pub async fn interact_with_open_ai(message: &str) -> Result<String, String> {
    let client = Client::new();
    let bearer = format!("Bearer {}", std::env::var("CHAT_API_KEY").unwrap());

    let request = CompletionRequest {
        model: "gpt-3.5-turbo".to_owned(),
        messages: vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant.".to_owned()
            },
            Message {
                role: "user".to_owned(),
                content: message.to_owned()
            }
        ],
        temperature: 0.7,
        n: 1,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions".to_string())
        .header(ACCEPT, "*/*")
        .header("OpenAI-Organization","org-ZFfZMGuCBWpCVT9jmtDaRE5p")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, &bearer)
        .json(&request)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<CompletionResponse>().await {
                Ok(parsed) => {
                    return Ok(parsed.choices[0].message.content.clone());
                },
                Err(_) => 
                    return Err(String::from("🛑 Hm, the response didn't match the shape we expected.")),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(String::from("🛑 Status: UNAUTHORIZED\n\rNeed to grab a new token"));
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            return Err(String::from("🛑 Status: 429 - TOO_MANY_REQUESTS\n\rNeed to Increase request limit"));
        }
        other => {
            return Err(String::from(
                format!("🛑 Uh oh! Something unexpected happened: [{:#?}]",
                other)
            ));
        }
    };
}