mod ai_structs;

use reqwest::blocking::Client;
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
fn main() {
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
                    let response = interact_with_chat_ai(&input);
                    write!(
                        stdout,
                        "{}{}{}{}\n\r",
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


fn interact_with_chat_ai(message: &str) -> String {
    // let client = Client::new();
    // let request = CompletionRequest {
    //     prompt: message.to_owned(),
    //     temperature: 0.7,
    //     max_tokens: 60,
    //     n: 1,
    //     stop: None,
    //     engine: "davinci".to_owned(),
    // };

    // let response = client
    // .post("https://api.openai.com/v1/engines/davinci/completions")
    // .header("Content-Type", "application/json"),
    // .header("Authorization", format!("Bearer {}", TOKEN))
    // .json(&request)
    // .send()
    // .unwrap();

    // let response_json: CompletionResponse = response.json().unwrap();
    // response_json.choices[0].text.clone()
    message.to_owned()
}