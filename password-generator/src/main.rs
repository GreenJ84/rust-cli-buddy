use std::io::{stdout, stdin, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;
use termion::color;
use termion::cursor::{Goto, Show, BlinkingUnderline, Left};
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
        // Get the desired password length
        let mut length_prompt = true;
        let mut length: u32 = 0;
        while length_prompt{
            write!(stdout,
                "{}{}{}How many digits would you like your password to be? (6-60)\n\r{}> {}",
                clear::All,
                Goto(1,1),
                color::Fg(color::Cyan),
                color::Fg(color::Red),
                color::Fg(color::Reset),
            ).unwrap();
            stdout.flush().unwrap();

            let mut input = String::new();
            for key in stdin().keys(){
                match key.unwrap() {
                    Key::Char('q') | Key::Esc => {
                        running = false;
                        length_prompt = false;
                        break;
                    },
                    Key::Char('\n') | Key::Char(' ') => {
                        write!(stdout, "\n\r").unwrap();
                        length_prompt = false;
                        break;
                    },
                    Key::Delete | Key::Backspace => {
                        input.pop();
                        write!(
                            stdout,
                            "{}{}",
                            Left(1),
                            clear::AfterCursor
                        ).unwrap();
                    }
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
                if let Ok(num) = eval_length(&input){
                    match num {
                        0..=7 | 61.. => {
                            write!(
                                stdout,
                                "\r{}{}{}{}",
                                clear::AfterCursor,
                                color::Fg(color::Red),
                                input,
                                color::Fg(color::Reset)
                            ).unwrap();
                        },
                        8..=12 => {
                            write!(
                                stdout,
                                "\r{}{}{}{}",
                                clear::AfterCursor,
                                color::Fg(color::Yellow),
                                input,
                                color::Fg(color::Reset)
                            ).unwrap();
                        },
                        13..=60 => {
                            write!(
                                stdout,
                                "\r{}{}{}{}",
                                clear::AfterCursor,
                                color::Fg(color::Green),
                                input,
                                color::Fg(color::Reset)
                            ).unwrap();
                        }
                    }
                } else {
                    write!(
                        stdout,
                        "\r{}{}{}{}",
                        clear::AfterCursor,
                        color::Fg(color::Red),
                        input,
                        color::Fg(color::Reset)
                    ).unwrap();
                }
                stdout.flush().unwrap();
            }
            if running && !length_prompt{
                if let Ok(num) = eval_length(&input) {
                    length = num;
                    break;
                } else{
                    length = 0;
                    length_prompt = true;
                    input.clear();
                    write!(
                        stdout,
                        "\n\r{}Length unable to be implemented.{}\n\n\r",
                        color::Fg(color::Red),
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout.flush().unwrap();
                    sleep(Duration::from_millis(800));
                }
            }
        }
        if !running{ break; }

        write!(stdout,
            "{}Is there a custom password portion you would like to enter? (within to your desired length)\n\r{}> {}",
            color::Fg(color::Cyan),
            color::Fg(color::Red),
            color::Fg(color::Reset),
        ).unwrap();
        stdout.flush().unwrap();

        let mut root = String::new();
        for key in stdin().keys(){
            stdout.flush().unwrap();
            match key.unwrap() {
                Key::Char('q') | Key::Esc => {
                    running = false;
                    break;
                },
                Key::Char('\n') | Key::Char(' ') => {
                    let password = generate_pasword(length, &root);
                    write!(
                        stdout,
                        "\n\rYour generated password is: {}{}{} \n\n\r",
                        color::Fg(color::Green),
                        password,
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout.flush().unwrap();
                    sleep(Duration::from_secs(5));
                    break;
                },
                Key::Delete | Key::Backspace => {
                    root.pop();
                    write!(
                        stdout,
                        "{}{}",
                        Left(1),
                        clear::AfterCursor
                    ).unwrap();
                },
                Key::Char(c) => {
                    root.push(c);
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
        "{}{}{}Shutting down..",
        clear::All,
        Goto(1,1),
        color::Fg(color::Red)
    ).unwrap();
    sleep(Duration::from_millis(200));
    for _ in 0..5{
        write!(
            stdout,
            "..",
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(200));
    }
    write!(
        stdout,
        "...{}Good Bye!{}",
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(500));
    return;
}

fn eval_length(length: &str) -> Result<u32, ()>{
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

fn generate_pasword(length: u32, root: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();

    let insert_index = rng.gen_range(0..=length - root.len() as u32 - 1);

    for i in 0..(length - root.len() as u32){
        if i == insert_index {
            password.push_str(root);
        }
        match rng.gen_range(0..120) % 7 {
            0 | 3 => {
                password.push(
                    std::char::from_digit(rng.gen_range(0..10), 10).unwrap()
                );
            },
            1 | 4 => {
                password.push(
                    std::char::from_u32(rng.gen_range(b'a'..b'z'+1) as u32).unwrap(),
                )
            },
            2 | 5 => {
                password.push(
                    std::char::from_u32(rng.gen_range(b'A'..b'Z'+1) as u32).unwrap(),
                )
            },
            _ => {
                let specials = vec!['!', '#', '$', '%', '&', '*', '_', '?'];
                password.push(
                    *specials.choose(&mut rng).unwrap()
                );
            },
        }
    }
    password
}