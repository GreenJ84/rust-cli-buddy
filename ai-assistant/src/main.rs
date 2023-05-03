mod ai_structs;


use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::io::{Write, stdin, stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::{color, cursor, clear, terminal_size};
use termion::event::Key;
use termion::cursor::DetectCursorPos;
use termion::input::TermRead;

use ai_structs::{CompletionRequest, CompletionResponse, CompletionChoices, LogProbs};

// Development companion, AI assistant
#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut stdout = stdout();
    write!(
        stdout,
        "{}{}{}Welcome to your AI Assistant!{}",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(1500));

    let mut running = true;
    while running {
        write!(
            stdout,
            "{}{}{}How can I be of service?\n\r{} > {}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Cyan),
            color::Fg(color::Red),
            color::Fg(color::Reset),
            cursor::Show,
            cursor::BlinkingUnderline,
        ).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        for key in stdin().keys(){
            let position = stdout.cursor_pos().unwrap();
            let _terminal = terminal_size().unwrap();
            match key.unwrap(){
                Key::Esc => {
                    running = false;
                    break;
                },
                Key::Delete | Key::Backspace => {
                    if input.len() > 0{
                        input.pop();
                        write!(
                            stdout,
                            "{}",
                            "\x08"
                        ).unwrap()
                    }
                },
                Key::Char('\n') => {
                    let response = interact_with_chat_ai(&input).await;
                    write!(
                        stdout,
                        "{}{}{:?}{}\n\r",
                        cursor::Goto(1, position.1 + 4),
                        color::Fg(color::Cyan),
                        response,
                        color::Fg(color::Reset),
                    ).unwrap();
                    break;
                },
                Key::Left => {
                    if position.1 > 1 && position.0 > 4 {
                        write!(
                            stdout,
                            "{}",
                            cursor::Left(1)
                        ).unwrap();
                    }
                },
                Key::Right => {
                    // write!(stdout, "{:?}", position).unwrap();
                    if stdout.cursor_pos().unwrap().0 - 4 < input.len() as u16 {
                        write!(
                            stdout, 
                            "{}", 
                            cursor::Right(1)
                        ).unwrap();
                    }
                },
                Key::Char(c) => {
                    input.push(c);
                    write!(
                        stdout,
                        "{}", c
                    ).unwrap();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }

        if running{
            write!(
                stdout,
                "{}Is there another question I can help with?\n\r {} > {}",
                color::Fg(color::Yellow),
                color::Fg(color::Red),
                color::Fg(color::Reset),
            ).unwrap();
            stdout.flush().unwrap();

            for key in stdin().keys(){
                match key.unwrap(){
                    Key::Esc | Key::Char('n') => {
                        running = false;
                        break;
                    },
                    Key::Char('y') | Key::Char('\n') => {
                        break;
                    },
                    _ => {}
                }
            }
        }
    }

    write!(
        stdout,
        "{}{}{}{}Closing AI Chat..",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Red),
        cursor::Hide,
    ).unwrap();
    for _ in 0..5{
        write!(
            stdout,
            "...",
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    write!(
        stdout,
        "{}Good Bye{}",
        color::Fg(color::Green),
        color::Fg(color::Reset),
    ).unwrap();
    stdout.flush().unwrap();
}


async fn interact_with_chat_ai(message: &str) -> String {
    let client = Client::new();
    let bearer = format!("Bearer {}", std::env::var("CHAT_API_KEY").unwrap());
    println!("{}", bearer);
    let mut request: String = r#"
    {
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "{}"
            }
        ]
    }"#.to_string();
    request = format!("{}", request.replace("{}", message));
    // let request = CompletionRequest {
    //     model: "gpt-3.5-turbo".to_owned(),
    //     prompt: message.to_owned(),
    //     max_tokens: 7,
    //     temperature: 0.7,
    //     n: 1,
    //     stream: false,
    // };

    let response = client
    .post("https://api.openai.com/v1/chat/completions".to_string())
    .header(ACCEPT, "*/*")
    .header("OpenAI-Organization","org-ZFfZMGuCBWpCVT9jmtDaRE5p")
    .header(CONTENT_TYPE, "application/json")
    .header(AUTHORIZATION, &bearer)
    .body(request)
    .send()
    .await
    .unwrap();

        write!(stdout(), "{:?}\n\r", response).unwrap();
        stdout().flush().unwrap();
    let response_json: CompletionResponse = response.json().await.unwrap();
        write!(stdout(), "{:?}\n\r", response_json).unwrap();
        stdout().flush().unwrap();
    let response: Vec<CompletionChoices> = response_json.choices;
        write!(stdout(), "{:?}\n\r", response).unwrap();
        stdout().flush().unwrap();
    if let Some(prob) = &response[0].logprobs {
        let logprobs: LogProbs = prob.clone();
        write!(stdout(), "\n\r{}-{}\n\r", logprobs.token_logprobs[0], logprobs.text_offset[0]).unwrap();
        stdout().flush().unwrap();
    }
    response[0].text.clone()
}

Response { url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("api.openai.com")), port: None, path: "/v1/chat/completions", query: None, fragment: None }, status: 200, headers: {"date": "Wed, 03 May 2023 02:47:12 GMT", "content-type": "application/json", "content-length": "319", "connection": "keep-alive", "access-control-allow-origin": "*", "cache-control": "no-cache, must-revalidate", "openai-model": "gpt-3.5-turbo-0301", "openai-organization": "user-sr9chuk2z67sv10w9xdptl8j", "openai-processing-ms": "654", "openai-version": "2020-10-01", "strict-transport-security": "max-age=15724800; includeSubDomains", "x-ratelimit-limit-requests": "3", "x-ratelimit-limit-tokens": "40000", "x-ratelimit-remaining-requests": "2", "x-ratelimit-remaining-tokens": "39973", "x-ratelimit-reset-requests": "20s", "x-ratelimit-reset-tokens": "40ms", "x-request-id": "383c8e9b625f2a0c93e158c5288501e9", "cf-cache-status": "DYNAMIC", "server": "cloudflare", "cf-ray": "7c152d89292c27b3-SEA", "alt-svc": "h3=\":443\"; ma=86400, h3-29=\":443\"; ma=86400"} }
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Decode, source: Error("missing field `text`", line: 1, column: 316) }', ai-assistant/src/main.rs:203:67
                                                                               note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
