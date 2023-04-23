use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::color;
use termion::clear;
use termion::cursor;

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




