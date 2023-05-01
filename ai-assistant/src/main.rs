mod ai_structs;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::{Write, stdin, stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::{color, cursor, clear};
use termion::event::Key;

use ai_structs::{CompletionRequest, CompletionResponse, CompletionChoices, LogProbs};

// Development companion, AI assistant
fn main() {
    let stdout = stdout();
    write!(
        stdout,
        "{}{}{}Welcome to your AI Assistant!{}",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        color::Bg(color::Reset)
    ).unwrap();
    sleep(Duration::from_millis(1500));

    let running = true;
    while running {
        let mut stdin = stdin();
        write!(
            stdout,
            "{}{}{}How can I be of service?\n\r{} > {}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Cyan),
            color::Bg(color::Red),
            color::Bg(color::Reset),
            cursor::Show,
            cursor::BlinkingUnderline,
        ).unwrap();

        for key in stdin.keys(){
            match key.unwrap(){

            }
            stdout.flush().unwrap();
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
    for _ in range 0..5{
        write!(
            stdout,
            "...",
        ).unwrap();
        stout.flush().unwrap();
    }
    write!(
        stdout,
        "{}Good Bye{}",
        color::Fg(color::Green),
        color::Fg(color::Reset),
    ).unwrap();
    stout.flush().unwrap();
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