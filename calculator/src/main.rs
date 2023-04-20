use std::io::{stdout, stdin, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::input::TermRead;
use termion::clear;
use termion::color;
use termion::style;
use termion::cursor::{Goto, Show, Left, BlinkingUnderline, DetectCursorPos};
use termion::event::Key;

fn main() {
    let mut stdout = stdout();
    write!(
        stdout,
        "{}{}{}Welcome to the {}Calculator!{}{}\n\r",
        clear::All,
        Goto(1,1),
        color::Fg(color::Green),
        style::Bold,
        Show,
        style::Reset
    ).unwrap();
    stdout.flush().unwrap();

    let mut recents_stack: Vec<String> = Vec::new();
    let mut offset: u32 = 0;
    let mut input = String::new();
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

        for key in stdin.keys() {
            stdout.flush().unwrap();
            match key.unwrap(){
                Key::Char('\n') => {
                    if let Ok(result) = eval(&input){
                        recents_stack.push(input.to_owned());
                        input = String::new();
                        offset = 0;
                        write!(
                            stdout,
                            "\n\r{}{}{}\n\n\r",
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
                Key::Up => {
                    let x = recents_stack.len();
                    let curr_line = stdout.cursor_pos().unwrap().1;
                    if (offset as usize) < recents_stack.len(){
                        offset += 1;
                        if let Some(item) = recents_stack.get(x.checked_sub((offset).try_into().unwrap()).unwrap_or_default()){
                            input = item.to_owned();
                        } else {
                            input = String::new();
                        }
                        write!(
                            stdout,
                            "{}{}{}{:?}{}",
                            Goto(3, curr_line),
                            clear::AfterCursor,
                            color::Fg(color::Yellow),
                            input,
                            color::Fg(color::Reset)
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
                },
                Key::Down => {
                    let curr_line = stdout.cursor_pos().unwrap().1;
                    let x = recents_stack.len();

                    if offset > 1{
                        offset -= 1;
                        if let Some(item) = recents_stack.get(x.checked_sub((offset).try_into().unwrap()).unwrap_or_default()){
                            input = item.to_owned();
                        } else {
                            input = String::new();
                        }
                        write!(
                            stdout,
                            "{}{}{}{:?}{}",
                            Goto(3, curr_line),
                            clear::AfterCursor,
                            color::Fg(color::Yellow),
                            input,
                            color::Fg(color::Reset)
                        ).unwrap();
                        stdout.flush().unwrap();
                    } else if offset == 1{
                        offset -= 1;
                        write!(
                            stdout,
                            "{}{}{}",
                            Goto(3, curr_line),
                            clear::AfterCursor,
                            color::Fg(color::Reset)
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
                },
                Key::Esc | Key::Char('q') => {
                    running = false;
                    break;
                },
                Key::Char(c) => {
                    input.push(c);
                    write!(stdout,"{}", c).unwrap();
                    stdout.flush().unwrap();
                },
                Key::Backspace | Key::Delete => {
                    if input.len() > 0 as usize{
                        input.pop();
                        write!(
                            stdout,
                            "{}{}",
                            Left(1),
                            clear::AfterCursor
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
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
                            let a = stack.pop().ok_or(())?;
                            stack.push(((a / b) as u16) as f64);
                        },
                        Some(&"%") => {
                            sign.pop();
                            let b = stack.pop().ok_or(())?;
                            let a = stack.pop().ok_or(())?;
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
