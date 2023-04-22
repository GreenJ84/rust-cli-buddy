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
        // Get the desired password length
        let mut length_prompt = true;
        while length_prompt{
            write!(stdout,
                "{}{}How many digits would you like your password to be? (8-60)\n\r{}> {}",
                clear::All,
                Goto(1,1),
                color::Fg(color::Red),
                color::Fg(color::Reset),
            ).unwrap();
            stdout.flush().unwrap();

            let mut length = String::new();
            for key in stdin.keys(){
                stdout.flush().unwrap();
                match key.unwrap() {
                    Key::Char('q') | Key::Esc => {
                        running = false;
                        length_prompt = false;
                        break;
                    },
                    Key::Char('\n') | Key::Char(' ') => {
                        if let Ok(num) = eval_length(&length) {
                            length_prompt = false;
                            break;
                    } else{
                            write!(
                                stdout,
                                "\n\r{}Length unable to be implemented.{}\n\n\r",
                                color::Fg(color::Red),
                                color::Fg(color::Reset),
                            ).unwrap();
                            stdout.flush().unwrap();
                        }
                        sleep(Duration::from_millis(800));
                        break;
                    },
                    Key::Char(c) => {
                        length.push(c);
                        write!(
                            stdout,
                            "{}", c,
                        ).unwrap();
                        stdout.flush().unwrap();
                    },
                    _ => {},
                }
                if let Ok(num) = eval_length(&length){
                    match num {
                        0..=10 | 61.. => {
                            write!(
                                stdout,
                                "\r{}{}{}{}",
                                clear::AfterCursor,
                                color::Fg(color::Red),
                                length,
                                color::Fg(color::Reset)
                            ).unwrap();
                        },
                        11..=15 => {
                            write!(
                                stdout,
                                "\r{}{}{}{}",
                                clear::AfterCursor,
                                color::Fg(color::Yellow),
                                length,
                                color::Fg(color::Reset)
                            ).unwrap();
                        },
                        16..=60 => {
                            write!(
                                stdout,
                                "\r{}{}{}{}",
                                clear::AfterCursor,
                                color::Fg(color::Green),
                                length,
                                color::Fg(color::Reset)
                            ).unwrap();
                        }
                    }
                }
                stdout.flush().unwrap();
            }
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
    for _ in 0..(length - root.len() as u32){
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