use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::{ color, cursor, event::Key, input::TermRead, style};

use ai_assistant as AI;
use buddy_service::utils::{application, terminal};

//# Future Implementations:
//? Add a way to have multiple chats, with management of chat history.
//? Inter program key bindings?

// Development companion, AI assistant
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Take inherited output
    let mut stdout = stdout();
    // Prgram entrance
    application::enter(&stdout, "Welcome to your AI Assistant!");
    sleep(Duration::from_millis(400));
    terminal::clear_terminal(&stdout);

    // Program running loop
    let mut running = true;
    while running {
        terminal::clear_terminal(&stdout);
        write!(
            stdout,
            "{}\n\r{}{}{}",
            terminal::buddy_text("How can I be of service?"),
            terminal::error_text(" > "),
            cursor::Show,
            cursor::BlinkingUnderline,
        )?;
        stdout.flush()?;

        let mut input = String::new();
        for key in stdin().keys() {
            match key? {
                Key::Esc => {
                    running = false;
                    break;
                }
                Key::Delete | Key::Backspace => {
                    if input.len() > 0 {
                        input.pop();
                        write!(stdout, "\x08 \x08",)?
                    }
                }
                Key::Char('\n') => {
                    write!(stdout, "\n\n\r")?;
                    stdout.flush()?;

                    let mut offset: String = String::new();
                    match AI::interact_with_open_ai(&input).await {
                        Ok(response) => {
                            write!(stdout, "{}", color::Fg(color::Green))?;

                            let mut in_code = false;
                            for line in response.split("\n") {
                                if line.starts_with("```") {
                                    in_code = !in_code;
                                    if in_code {
                                        write!(
                                            stdout,
                                            "\t{}{}{}{}\n\r",
                                            color::Fg(color::LightMagenta),
                                            style::Bold,
                                            line,
                                            color::Fg(color::LightBlue)
                                        )?;
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
                                        )?;
                                    }
                                } else {
                                    write!(stdout, "{}{}\n\r", offset, line,)?;
                                }
                                stdout.flush()?;
                            }
                        }
                        Err(problem) => {
                            application::error_confirmation(&mut stdout, &problem);
                            // running = false;
                        }
                    }
                    write!(stdout, "\n\n\r{}", color::Fg(color::Reset))?;
                    stdout.flush()?;
                    break;
                }
                Key::Char(c) => {
                    input.push(c);
                    write!(stdout, "{}", c)?;
                }
                _ => {}
            }
            stdout.flush()?;
        }

        if running {
            running =
                application::message_confirmation(&mut stdout, "Is there more I can help with?");
        }
    }

    application::exit(&stdout, "Closing AI Assistant", "Good Bye");
    Ok(())
}
