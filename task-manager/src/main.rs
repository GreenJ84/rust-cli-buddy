use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::clear;
use termion::color;
use termion::cursor;
use termion::style;
use termion::input::TermRead;

// Add, manage, delete tasks. task timelines, deadlines, and updates
fn main() {
    let mut stdout = stdout();
    write!(
        stdout,
        "{}{}{}Welcome to the Task Manager!{}",
        clear::All,
        cursor::Goto(1,1),
        color::Fg(color::Green),
        color::Fg(color::Reset),
    ).unwrap();

    let mut running = false;
    let first = true;
    while running{
        let stdin = stdin();
        write!(
            stdout,
            "{}{}{}How can I help you {}?{}\n\r",
            clear::All,
            cursor::Goto(1,1),
            color::Fg(color::Cyan),
            if first {"today"} else {"next"},
            color::Fg(color::Reset),
        ).unwrap();

        let options: [&str; 4] = [
            "'N'ew Task",
            "'R'eview Task",
            "'U'pdate Task",
            "'D'elete Task",
        ];

        let mut selected = 0;
        for (idx, option) in options.iter().enumerate(){
            if selected == idx{
                write!(
                    stdout,
                    "",
                    color::Fg(color::Red),
                    option
                ).unwrap();
            } else{
                write!(
                    stdout,
                    "",
                    color::Fg(color::Red),
                    option,
                ).unwrap();
            }
        }

        first = false;
    }

    write!(
        stdout,
        "Closing Task Manager..."
    ).unwrap();
    for _ in 0..5{
        write!(
            stdout,
            ".."
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    write!(
        stdout,
        "...BYE",
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(500));
}
