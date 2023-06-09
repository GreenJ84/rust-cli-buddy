mod ai_structs;


use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};
use std::io::{Write, stdin, stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::{color, cursor, clear, style, terminal_size};
use termion::event::Key;
use termion::cursor::DetectCursorPos;
use termion::input::TermRead;

use ai_structs::{CompletionRequest, CompletionResponse, Message};

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
            let _position = stdout.cursor_pos().unwrap();
            let _terminal = terminal_size().unwrap();
            match key.unwrap(){
                Key::Esc => {
                    running = false;
                    break;
                },
                Key::Delete | Key::Backspace => {
                    if input.len() > 0 {
                        input.pop();
                        write!(
                            stdout,
                            "\x08 \x08",
                        ).unwrap()
                    }
                },
                Key::Char('\n') => {
                    write!(stdout,"\n\n\r").unwrap();
                    stdout.flush().unwrap();
                    let mut offset: String = String::new();
                    match interact_with_chat_ai(&input).await{
                        Ok(response) => {
                            write!(stdout, "{}", color::Fg(color::Green)).unwrap();
                            stdout.flush().unwrap();
                            let mut in_code = false;
                            for line in response.split("\n"){
                                if line.starts_with("```"){
                                    in_code = !in_code;
                                    if in_code {
                                        write!(
                                            stdout,
                                            "{}{}    {}{}\n\r",
                                            color::Fg(color::LightMagenta),
                                            style::Bold,
                                            line,
                                            color::Fg(color::LightBlue)
                                        ).unwrap();
                                        offset = String::from("\t"); 
                                    } else {
                                        offset = String::new();
                                        write!(
                                            stdout,
                                            "{}    {}{}{}\n\r",
                                            color::Fg(color::LightMagenta),
                                            line,
                                            style::Reset,
                                            color::Fg(color::Green),
                                        ).unwrap();
                                    }
                                } else{
                                    write!(
                                        stdout,
                                        "{}{}\n\r",
                                        offset,
                                        line,
                                    ).unwrap();
                                }
                                stdout.flush().unwrap();
                            }
                        },
                        Err(problem) => 
                            write!(
                                stdout, 
                                "{}{}{}",
                                color::Fg(color::Red),
                                problem,
                                color::Fg(color::Reset),
                            ).unwrap(),
                    }
                    write!(stdout,"\n\n\r").unwrap();
                    stdout.flush().unwrap();
                    break;
                },
                // Key::Left => {
                //     if position.1 > 1 && position.0 > 4 {
                //         write!(
                //             stdout,
                //             "{}",
                //             cursor::Left(1)
                //         ).unwrap();
                //     }
                // },
                // Key::Right => {
                //     if stdout.cursor_pos().unwrap().0 - 4 < input.len() as u16 {
                //         write!(
                //             stdout, 
                //             "{}", 
                //             cursor::Right(1)
                //         ).unwrap();
                //     }
                // },
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
                "{}Is there more I can help with? y/n\n\r {} > {}",
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
                    _ => {
                        write!(
                            stdout,
                            "{}Not understood. Try again{}",
                            color::Fg(color::Yellow),
                            color::Fg(color::Reset),
                        ).unwrap();
                        stdout.flush().unwrap();
                        sleep(Duration::from_millis(800));
                        write!(
                            stdout,
                            "\r{}",
                            clear::CurrentLine,
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
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


async fn interact_with_chat_ai(message: &str) -> Result<String, String> {
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
            return Err(String::from("🛑 Status: UNAUTHORIZED - Need to grab a new token"));
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            return Err(String::from("🛑 Status: 429 - Too many requests"));
        }
        other => {
            panic!(
                "🛑 Uh oh! Something unexpected happened: [{:#?}]", 
                other
            );
        }
    };
}
