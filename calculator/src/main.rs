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
        "{}{}Welcome to the calculator!{}",
        clear::All,
        Goto(1,1),
        Show
    ).unwrap();

    let mut running = true;
    while running {
        let stdin = stdin();
        write!(
            stdout,
            "{}\r{}> {}{}",
            clear::CurrentLine,
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
                Key::Backspace | Key::Delete => {
                    input.pop();
                    write!(
                        stdout,
                        "{}{}",
                        Left(1),
                        clear::AfterCursor
                    ).unwrap();
                    stdout.flush().unwrap();
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
    let segments: Vec<&str> = expr.split_ascii_whitespace().collect();

    let mut stack: Vec<f64> = Vec::new();
    let mut sign: Vec<&str> = Vec::new();
    for item in segments{
        match item {
            "+" => {
                sign.push(item);
            },
            "-" => {
                sign.push(item);
            },
            "*" => {
                sign.push(item);
            },
            "**" => {
                sign.push(item);
            },
            "//" => {
                sign.push(item);
            },
            "%" => {
                sign.push(item);
            },
            "/" => {
                sign.push(item);
            },
            _ => {
                let num = item.parse::<f64>().map_err(|_| ())?;
                stack.push(num);
                if sign.len() > 0{
                    match sign.last() {
                        Some(&"*") => {
                            sign.pop();
                            let b = stack.pop().ok_or(())?;
                            let a = stack.pop().ok_or(())?;
                            stack.push(a * b);
                        },
                        Some(&"**") => {
                            sign.pop();
                            let b = stack.pop().ok_or(())?;
                            let mut a = stack.pop().ok_or(())?;
                            for _ in 1..b as u16 {
                                a *= a
                            }
                            stack.push(a);
                        },
                        Some(&"//") => {
                            sign.pop();
                            let b = stack.pop().ok_or(())?;
                            let mut a = stack.pop().ok_or(())?;
                            stack.push(((a / b) as u16) as f64);
                        },
                        Some(&"%") => {
                            sign.pop();
                            let b = stack.pop().ok_or(())?;
                            let mut a = stack.pop().ok_or(())?;
                            stack.push(a % b);
                        },
                        Some(&"/") => {
                            sign.pop();
                            let b = stack.pop().ok_or(())?;
                            let a = stack.pop().ok_or(())?;
                            stack.push(a / b);
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    while sign.len() > 0{
        let b = stack.pop().ok_or(())?;
        let a = stack.pop().ok_or(())?;
        match sign.last() {
            Some(&"+") => {
                stack.push(a + b);
            },
            Some(&"-") => {
                stack.push(a - b);
            },
            _ => {
                return Err(());
            }
        }
        sign.pop();
    }

    if stack.len() == 1{
        Ok(stack[0])
    } else{
        Err(())
    }
}
