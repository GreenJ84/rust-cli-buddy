use std::io::{stdout, stdin, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;
use termion::color;
use termion::cursor::{Goto, Show, BlinkingUnderline};
use termion::event::Key;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}Welcome to the password generator!\n\r{}{}{}",
        clear::All,
        Goto(1,1),
        color::Fg(color::Green),
        color::Fg(color::Reset),
        Show,
        BlinkingUnderline,
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(1));

    let mut running = true;
    while running {
        let stdin = stdin();
        write!(stdout,
            "How many digits would you like your password to be?\n\r{}> {}",
            color::Fg(color::Red),
            color::Fg(color::Reset),
        ).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        for key in stdin.keys(){
            stdout.flush().unwrap();
            match key.unwrap() {
                Key::Char('q') | Key::Esc => {
                    running = false;
                    break;
                },
                Key::Char('\n') | Key::Char(' ') => {
                    if let Ok(num) = eval_length(input) {
                        let password = generate_pasword(num);
                        write!(
                            stdout,
                            "\n\rYour generated password is: {}{}{} \n\n\r",
                            color::Fg(color::Green),
                            password,
                            color::Fg(color::Reset),
                        ).unwrap();
                        stdout.flush().unwrap();
                } else{
                        write!(
                            stdout,
                            "\n\r{}Length unable to be implemented.{}\n\n\r",
                            color::Fg(color::Red),
                            color::Fg(color::Reset),
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
                    sleep(Duration::from_secs(1));
                    break;
                },
                Key::Char(c) => {
                    input.push(c);
                    write!(
                        stdout,
                        "{}", c,
                    ).unwrap();
                    stdout.flush().unwrap();
                },
                _ => {},
            }
            stdout.flush().unwrap();
        }
        if !running{ break; }
    }

    write!(
        stdout,
        "{}{}{}...password generator shutting down.{}",
        clear::All,
        Goto(1,1),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();

    sleep(Duration::from_secs(2));
    return;
}

fn eval_length(length: String) -> Result<u32, ()>{
    if let Ok(num) = length.trim().parse::<u32>(){
        if num < 61 && num > 7{
            Ok(num)
        } else{
            Err(())
        }
    } else{
        Err(())
    }
}

fn generate_pasword(length: u32) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    for _ in 0..length{
        match rng.gen_range(0..120) % 4 {
            0 => {
                password.push(
                    std::char::from_digit(rng.gen_range(0..10), 10).unwrap()
                );
            },
            1 => {
                password.push(
                    std::char::from_u32(rng.gen_range(b'a'..b'z'+1) as u32).unwrap(),
                )
            },
            2 => {
                password.push(
                    std::char::from_u32(rng.gen_range(b'A'..b'Z'+1) as u32).unwrap(),
                )
            },
            _ => {
                let specials = vec!['!', '@', '#', '$', '%', '^', '&', '*', '_', '?'];
                password.push(
                    *specials.choose(&mut rng).unwrap()
                );
            },
        }
        
    }
    password
}