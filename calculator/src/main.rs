use buddy_utils::{application_close, application_entry};

use std::io::{stdout, stdin, Write, Stdout, Stdin};
use std::thread::sleep;
use std::time::Duration;
use termion::input::TermRead;
use termion::clear;
use termion::color;
use termion::cursor::{Goto, Left, BlinkingUnderline, DetectCursorPos};
use termion::event::Key;

fn main() {
    let mut stdout: Stdout = stdout();
    application_entry(&stdout, "Welcome to the Calculator!");

    let mut recents_stack: Vec<String> = Vec::new();
    let mut offset: u32 = 0;
    let mut input = String::new();
    let mut running: bool = true;
    while running {
        write!(
            stdout,
            "\n{}\r{}> {}{}",
            clear::CurrentLine,
            color::Fg(color::Red),
            color::Fg(color::Reset),
            BlinkingUnderline
        ).unwrap();
        stdout.flush().unwrap();
        
        let stdin: Stdin = stdin();
        for key in stdin.keys() {
            match key.unwrap(){
                Key::Char('\n') => {
                    if let Ok(result) = eval(&input){
                        recents_stack.push(input.to_owned());
                        offset = 0;
                        write!(
                            stdout,
                            "\n\r{} = {}{}\n\n\r",
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
                    input = String::new();
                    break;
                },
                Key::Up => {
                    let curr_line = stdout.cursor_pos().unwrap().1;
                    let x = recents_stack.len();

                    if (offset as usize) < recents_stack.len() {
                        offset += 1;
                        if let Some(item) = recents_stack.get(x.checked_sub((offset).try_into().unwrap()).unwrap_or_default()){
                            input = item.to_owned();
                        } else {
                            offset = 0;
                            input = String::new();
                        }
                    } else if (offset as usize) == recents_stack.len(){
                        offset = 0;
                        input = String::new();
                    }
                    write_response(&mut stdout, curr_line, &input);
                },
                Key::Down => {
                    let curr_line = stdout.cursor_pos().unwrap().1;
                    let x = recents_stack.len();

                    if offset > 1 || offset == 0{
                        offset = if offset == 0 { x as u32 } else { offset - 1 };
                        if let Some(item) = recents_stack.get(x.checked_sub((offset).try_into().unwrap()).unwrap_or_default()){
                            input = item.to_owned();
                        }
                    } else {
                        offset -= 1;
                        input = String::new();
                    }
                    write_response(&mut stdout, curr_line, &input);
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

    application_close(&stdout, "Exiting", "calculate ya later.");
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

fn write_response(stdout: &mut Stdout, curr_line: u16, message: &str){
    write!(
        stdout,
        "{}{}{}{}",
        Goto(3, curr_line),
        clear::AfterCursor,
        color::Fg(color::Reset),
        message
    ).unwrap();
    stdout.flush().unwrap();
}