// Move, Copy, Rename, Create, Delete, Sort, search, filter
use buddy_utils::{application_entry, application_close};
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, Stdout, stdin, Stdin};
use termion::color;
use termion::cursor;
use termion::clear;



const FILE_OPTIONS: [(&str, &str); 8] = [
    ("mv", "Move"),
    ("cp", "Copy"),
    ("rn", "Rename"),
    ("mk", "Create"),
    ("dl", "Delete"),
    ("so", "Sort"),
    ("se", "Search"),
    ("fi", "Filter")
];

fn main() {
    let mut stdout: Stdout = stdout();
    application_entry(&stdout, "Welcome to the File Manager!");

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
        display_options(&mut stdout, selected);


        let mut input: String = String::new();
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') | Key::Esc => {}
                Key::Char(' ') | Key::Char('\n') => {}
                Key::Up => {}
                Key::Down => {}
                Key::Char(c) => {
                    input.push(c);
                }
                _ => {}
            }
        }
    }


    application_close(&stdout, "", "");
}

fn display_options(stdout: &mut Stdout, selected: u32) {
    for (idx, option) in FILE_OPTIONS.iter().enumerate() {
        if idx == selected as usize {
            write!(
                stdout,
                "{}{}{}  > {}{}",
                cursor::Goto(1, idx as u16 + 2),
                clear::CurrentLine,
                color::Fg(color::Red),
                format!("{} - {}", option.0, option.1),
                color::Fg(color::Reset),
            ).unwrap();
        } else {
            write!(
                stdout,
                "{}{}{}    {}",
                cursor::Goto(1, idx as u16 + 2),
                clear::CurrentLine,
                color::Fg(color::Reset),
                format!("{} - {}", option.0, option.1),
            ).unwrap();
        }
        stdout.flush().unwrap();
    }
}

