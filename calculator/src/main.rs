use std::io::{stdout, stdin, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;
use termion::color;
use termion::cursor::{Goto, Show, Left, BlinkingUnderline};
use termion::event::Key;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}",
        Show,
        clear::All,
        Goto(1,1),
    ).unwrap();

    let mut running = true;
    while running {
        let stdin = stdin();
        write!(
            stdout,
            "\r{}> {}{}",
            color::Fg(color::Red),
            color::Fg(color::Reset),
            BlinkingUnderline
        ).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') | Key::Esc => {
                    stdout.flush().unwrap();
                    running = false;
                    break;
                },
                Key::Char('\n') => {
                    input.push(' ');
                    print!("\n\r");
                    if let Ok(result) = eval(&input){
                        stdout.flush().unwrap();
                        write!(
                            stdout,
                            "{}{}{}\n\n\r",
                            color::Fg(color::Green),
                            result,
                            color::Fg(color::Reset),
                        ).unwrap();
                        stdout.flush().unwrap();
                        sleep(Duration::from_secs(1));
                    } else{
                        write!(
                            stdout,
                            "{}Expression unable to evaluate.{}\n\n\r",
                            color::Fg(color::Red),
                            color::Fg(color::Reset),
                        ).unwrap();
                        stdout.flush().unwrap();
                        sleep(Duration::from_secs(1));
                    }
                    break;
                },
                Key::Char(c) => {
                    input.push(c);
                    write!(stdout,"{}", c).unwrap();
                    stdout.flush().unwrap();
                },
                Key::Backspace => {
                    input.pop();
                    write!(
                        stdout,
                        "{}{}",
                        Left(1),
                        clear::AfterCursor
                    ).unwrap();
                },
                _ => {}
            }
        }
        stdout.flush().unwrap();
        if !running{ break; }
    }

    write!(
        stdout,
        "{}{}{}...calculator shutting down.{}",
        clear::All,
        Goto(1,1),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();

    sleep(Duration::from_secs(2));
    return;
}

fn eval(expr: &str) -> Result<f64, ()> {
    Ok(1)
}
