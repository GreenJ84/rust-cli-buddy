use buddy_utils::{application_close, application_entry};

use std::io::{stdin, Stdin, stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::color;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;

const TIMER_OPTIONS: [&str; 4] = [
    "Stopwatch",
    "Work Intervals",
    "Timer",
    "Alarm"
];


// stopwatch, work interval, timer, alarms
fn main() {
    let mut stdout: Stdout = stdout();
    application_entry(&stdout, "Welcome to the Development Timer!");

    let mut running: bool = true;
    while running {
        let stdin: Stdin = stdin();

        write!(
            stdout,
            "{}{}{}How can I assist?{}{}",
            clear::All,
            cursor::Goto(1,1),
            color::Fg(color::Cyan),
            color::Fg(color::Reset),
            cursor::Hide
        ).unwrap();

        let mut selected: u32 = 0;
        for (idx, option) in TIMER_OPTIONS.iter().enumerate(){
            if idx == selected as usize {
                write!(
                    stdout,
                    "{}{}{}  > {}{}",
                    cursor::Goto(1, idx as u16 + 2),
                    clear::CurrentLine,
                    color::Fg(color::Red),
                    option.to_uppercase(),
                    color::Fg(color::Reset),
                ).unwrap();
            } else {
                write!(
                    stdout,
                    "{}{}{}    {}",
                    cursor::Goto(1, idx as u16 + 2),
                    clear::CurrentLine,
                    color::Fg(color::Reset),
                    option,
                ).unwrap();
            }
            stdout.flush().unwrap();
        }

        for key in stdin.keys(){
            match key.unwrap(){
                Key::Esc | Key::Char('q') => {
                    running = false;
                    break;
                },
                Key::Char(' ') | Key::Char('\n') => {
                    match selected {
                        0 => {
                            write!(stdout, "{}", "Starting the stopwatch...").unwrap();
                        },
                        1 => {
                            write!(stdout, "{}", "Starting the work intervals...").unwrap();
                        },
                        2 => {
                            write!(stdout, "{}", "Starting the timer...").unwrap();
                        },
                        _ => {
                            write!(stdout, "{}", "Starting the alarm...").unwrap();
                        }
                    }
                    write!(
                        stdout,
                        "{}{}{}Opening: {}{}{}",
                        clear::All,
                        cursor::Goto(1, 1),
                        color::Fg(color::Green),
                        color::Fg(color::Cyan),
                        TIMER_OPTIONS[selected as usize],
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout.flush().unwrap();
                    break;
                },
                Key::Up => {
                    if selected > 0 {
                        selected -= 1;
                        display_options(&stdout, selected);
                    }
                },
                Key::Down => {
                    if (selected as usize) < TIMER_OPTIONS.len() - 1 {
                        selected += 1;
                        display_options(&stdout, selected);
                    }
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }

        stdout.flush().unwrap();
        sleep(Duration::from_millis(800));
    }

    application_close(&stdout, "Closing up Development Timer", "DEVELOPER OUT!");
}

fn display_options(mut stdout: &Stdout, selected: u32) {
    for (idx, option) in TIMER_OPTIONS.iter().enumerate(){
        if idx == selected as usize {
            write!(
                stdout,
                "{}{}{}  > {}{}",
                cursor::Goto(1, idx as u16 + 2),
                clear::CurrentLine,
                color::Fg(color::Red),
                option.to_uppercase(),
                color::Fg(color::Reset),
            ).unwrap();
        } else {
            write!(
                stdout,
                "{}{}{}    {}",
                cursor::Goto(1, idx as u16 + 2),
                clear::CurrentLine,
                color::Fg(color::Reset),
                option,
            ).unwrap();
        }
        stdout.flush().unwrap();
    }
}

