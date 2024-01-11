use ai_assistant::gpt_structs::{CompletionRequest, CompletionResponse, Message};
use ai_assistant::{interact_with_open_ai};
use buddy_service::utils::{application, terminal};

use dotenv::dotenv;
use std::io::{Write, stdin, stdout};
use std::thread::sleep;
use std::time::Duration;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};
use termion::{color, cursor, clear, style, event::Key, input::TermRead};


// Development companion, AI assistant
#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut stdout = stdout();
    application::enter(&stdout, "Welcome to your AI Assistant!");
    sleep(Duration::from_millis(400));
    terminal::clear_terminal(&stdout);

    let mut running = true;
    while running {
        write!(
            stdout,
            "{}\n\r{} > {}{}",
            terminal::buddy_text_text("How can I be of service?"),
            terminal::error_text_text(" > "),
            cursor::Show,
            cursor::BlinkingUnderline,
        ).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        for key in stdin().keys(){
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
                    match ai_assistant::interact_with_open_ai(&input).await{
                        Ok(response) => {
                            write!(stdout, "{}", color::Fg(color::Green)).unwrap();

                            let mut in_code = false;
                            for line in response.split("\n"){
                                if line.starts_with("```"){
                                    in_code = !in_code;
                                    if in_code {
                                        write!(
                                            stdout,
                                            "\t{}{}{}{}\n\r",
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
                                            "\t{}{}{}{}\n\r",
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
                "{}Is there more I can help with? y/n {} > {}",
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
                        write!(
                            stdout,
                            "\n\n\r",
                        ).unwrap();
                        stdout.flush().unwrap();
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

    application::exit(&stdout, "Closing AI Assistant", "Good Bye");
}

