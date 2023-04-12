use std::io::{stdout, stdin, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;
use termion::color;
use termion::cursor::{Goto, Show, Left, BlinkingUnderline};
use termion::event::Key;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}Welcome to the password generator!{}{}",
        clear::All,
        Goto(1,1),
        Show,
        BlinkingUnderline,
    ).unwrap();

    let mut running = true;
    while running {
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
