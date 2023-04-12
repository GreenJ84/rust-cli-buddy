use std::io::{stdout, stdin, Write};
use std::process::{Command, Stdio};
use termion::clear;
use termion::color;
use termion::cursor::{Goto, Hide, Show};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::time::Duration;
use std::thread::sleep;

const BUDDY_PROGRAMS: [&str; 7] = [
    "calculator",
    "development-timer",
    "file-organizer",
    "password-manager",
    "password-generator",
    "task-manager",
    "word analyzer",
];

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut running = true;
    let mut selected = 0;
    
    while running {
        let stdin = stdin();

        write!(
            stdout, 
            "{}{}",
            clear::All,
            Hide,
        ).unwrap();

        write!(
            stdout, 
            "{}{}Select a program to start: {}", 
            Goto(1, 1),
            color::Fg(color::Green),
            color::Fg(color::Reset)
        ).unwrap();
        write!(
            stdout, 
            "{}{}enter / 'q'uit {}\n", 
            Goto(4, 2),
            color::Fg(color::Yellow),
            color::Fg(color::Reset)
        ).unwrap();

        // Print the list
        for (i, program) in BUDDY_PROGRAMS.iter().enumerate(){
            if selected == i {
                write!(
                    stdout,
                    "{}{}> {}{}",
                    Goto(1, (i+4) as u16),
                    color::Fg(color::Red),
                    program,
                    color::Fg(color::Reset),
                ).unwrap();
            } else {
                write!(
                    stdout,
                    "{}{}{}",
                    Goto(1, (i+4) as u16),
                    color::Fg(color::Reset),
                    program,
                ).unwrap();
            }
        }
        stdout.flush().unwrap();

        // Handle User Input
        for c in stdin.keys() {
            match c.unwrap(){
                Key::Up => {
                    if selected > 0{
                        write!(
                            stdout,
                            "{}{}{}{}",
                            Goto(1, selected as u16 + 4),
                            clear::CurrentLine,
                            color::Fg(color::Reset),
                            BUDDY_PROGRAMS[selected]
                        ).unwrap();
                        selected -= 1;
                        write!(
                            stdout,
                            "{}{}{}> {}{}",
                            Goto(1, selected as u16 + 4),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            BUDDY_PROGRAMS[selected],
                            color::Fg(color::Reset),
                        ).unwrap();
                    }
                },
                Key::Down => {
                    if selected < BUDDY_PROGRAMS.len() - 1{
                        write!(
                            stdout,
                            "{}{}{}{}",
                            Goto(1, selected as u16 + 4),
                            clear::CurrentLine,
                            color::Fg(color::Reset),
                            BUDDY_PROGRAMS[selected],
                        ).unwrap();
                        selected += 1;
                        write!(
                            stdout,
                            "{}{}{}> {}{}",
                            Goto(1, selected as u16 + 4),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            BUDDY_PROGRAMS[selected],
                            color::Fg(color::Reset),
                        ).unwrap();
                    }
                },
                Key::Char('\n') => {
                    write!(
                        stdout, 
                        "{}{}........Selecting{}",
                        Goto(BUDDY_PROGRAMS[selected].len() as u16 + 3, selected as u16 + 4),
                        color::Fg(color::Red),
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout.flush().unwrap();
                    sleep(Duration::from_secs(1));

                    write!(
                        stdout,
                        "{}{}{}You have chosesen: {}{}",
                        clear::All,
                        Goto(1, 1),
                        color::Fg(color::Green),
                        BUDDY_PROGRAMS[selected],
                        Goto(1, 2),
                    ).unwrap();
                    stdout.flush().unwrap();
                    sleep(Duration::from_secs(1));
                    break;
                },
                Key::Char('q') => {
                    write!(
                        stdout,
                        "{}{}{}Quitting program....",
                        clear::All,
                        Goto(1,1),
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout.flush().unwrap();
                    running = false;
                    sleep(Duration::from_secs(1));
                    break;
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }

        if !running { break; }

        println!("Starting {}", &BUDDY_PROGRAMS[selected]);
        sleep(Duration::from_secs(3));
        
        let mut spawn = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(BUDDY_PROGRAMS[selected])
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn application");

        let exit_status = spawn.wait().expect(&format!("Failed to wait for {} program.", BUDDY_PROGRAMS[selected]));
        println!(
            "{}{} Exiting {}. Status: {}",
            clear::All, 
            Goto(1,1),
            BUDDY_PROGRAMS[selected], 
            exit_status);
        sleep(Duration::from_secs(1));
    }

}
