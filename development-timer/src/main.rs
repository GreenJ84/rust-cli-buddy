use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::color;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;

// stopwatch, work interval, timer, alarms
fn main() {
    let mut stdout = stdout();
    write!(
        stdout,
        "{}{}{}Welcome to the Development Timer!{}",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(1500));

    let mut running = true;
    while running {
        let stdin = stdin();

        write!(
            stdout,
            "{}{}{}How can I assist?{}{}",
            clear::All,
            cursor::Goto(1,1),
            color::Fg(color::Cyan),
            color::Fg(color::Reset),
            cursor::Hide
        ).unwrap();
        let options: [&str; 4] = [
            "Stopwatch",
            "Work Intervals",
            "Timer",
            "Alarm"
        ];
        let mut selected: u32 = 0;
        for (idx, option) in options.iter().enumerate(){
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

                        },
                        1 => {

                        },
                        2 => {

                        },
                        _ => {

                        }
                    }
                    write!(
                        stdout,
                        "{}Opening: {}{}{}",
                        cursor::Goto(1, options.len() as u16 + 3),
                        color::Fg(color::Green),
                        options[selected as usize],
                        color::Fg(color::Reset),
                    ).unwrap();
                    break;
                },
                Key::Up => {
                    if selected > 0 {
                        write!(
                            stdout,
                            "{}{}{}    {}",
                            clear::CurrentLine,
                            cursor::Goto(1, selected as u16 + 2),
                            color::Fg(color::Reset),
                            options[selected as usize],
                        ).unwrap();
                        selected -= 1;
                        write!(
                            stdout,
                            "{}{}{}  > {}",
                            clear::CurrentLine,
                            cursor::Goto(1, selected as u16 + 2),
                            color::Fg(color::Reset),
                            options[selected as usize].to_uppercase(),
                        ).unwrap();
                    }
                },
                Key::Down => {
                    if (selected as usize) < options.len() - 1 {
                        write!(
                            stdout,
                            "{}{}{}    {}",
                            clear::CurrentLine,
                            cursor::Goto(1, selected as u16 + 2),
                            color::Fg(color::Reset),
                            options[selected as usize],
                        ).unwrap();
                        selected += 1;
                        write!(
                            stdout,
                            "{}{}{}  > {}",
                            clear::CurrentLine,
                            cursor::Goto(1, selected as u16 + 2),
                            color::Fg(color::Reset),
                            options[selected as usize].to_uppercase(),
                        ).unwrap();
                    }
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }

        write!(
            stdout,
            "{}{}Doing timer things",
            clear::All,
            cursor::Goto(1,1),
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(800));
        running = false;
    }

    write!(
        stdout,
        "{}{}{}{}Closing up Development Timer..",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Red),
        cursor::Hide,
    ).unwrap();
    for _ in 0..4{
        write!(
            stdout,
            "..."
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(200));
    }
    write!(
        stdout,
        "...{}DEVELOPERS OUT!{}",
        color::Fg(color::Green),
        color::Fg(color::Reset),
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(500));
}




