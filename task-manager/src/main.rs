use chrono::{DateTime, Utc};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use rusqlite::{Connection};

use buddy_utils::format_name;

#[derive(Debug)]
struct Task {
    id: Option<u64>,
    title: String,
    description: String,
    due_date: Option<DateTime<Utc>>,
    priority: u32,
    status: String,
    created_at: Option<DateTime<Utc>>, // SQL set
    updated_at: Option<DateTime<Utc>>, // SQL set
    completed_at: Option<DateTime<Utc>>
}

impl Task {
    fn new(
        title: String,
        description: String,
        due_date: Option<DateTime<Utc>>,
        priority: u32,
        status: String,
    ) -> Self {
        Self{
            id: None,
            title,
            description,
            due_date,
            priority,
            status,
            created_at: None,
            updated_at: None,
            completed_at: None
        }
    }
}


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

    let mut running = true;
    let mut first = true;
    while running{
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

        let mut selected: u16 = 0;
        for (idx, option) in options.iter().enumerate(){
            if (selected as usize) == idx{
                write!(
                    stdout,
                    "{}{}{}  > {}{}",
                    cursor::Goto(1, idx as u16 + 2),
                    clear::CurrentLine,
                    color::Fg(color::Red),
                    format_name(option).to_uppercase(),
                    color::Fg(color::Reset),
                ).unwrap();
            } else{
                write!(
                    stdout,
                    "{}{}{}    {}",
                    cursor::Goto(1, idx as u16 + 2),
                    clear::CurrentLine,
                    color::Fg(color::Reset),
                    format_name(option),
                ).unwrap();
            }
            stdout.flush().unwrap();
        }

        let conn = Connection::open("../../tasks_db.db3").unwrap();
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Char('q') | Key::Esc => {
                    running = false;
                    break;
                },
                Key::Char('\n') => {
                    match selected {
                        0 => {
                            new_task(&conn);
                        },
                        1 => {
                            retrieve_task(&conn);
                        },
                        2 => {
                            update_task(&conn);
                        },
                        _ => {
                            delete_task(&conn);
                        }
                    }
                    break;
                },
                Key::Up => {
                    if selected > 0 {
                        write!(
                            stdout, 
                            "{}{}{}    {}",
                            cursor::Goto(1, selected + 2),
                            clear::CurrentLine,
                            color::Fg(color::Reset),
                            format_name(options[selected as usize]),
                        ).unwrap();
                        selected -= 1;
                        write!(
                            stdout, 
                            "{}{}{}  > {}{}",
                            cursor::Goto(1, selected + 2),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            format_name(options[selected as usize]),
                            color::Fg(color::Reset),
                        ).unwrap();
                    }
                },
                Key::Down => {
                    if (selected as usize) < options.len() - 1 {
                        write!(
                            stdout, 
                            "{}{}{}    {}",
                            cursor::Goto(1, selected + 2),
                            clear::CurrentLine,
                            color::Fg(color::Reset),
                            format_name(options[selected as usize]),
                        ).unwrap();
                        selected += 1;
                        write!(
                            stdout, 
                            "{}{}{}  > {}{}",
                            cursor::Goto(1, selected + 2),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            format_name(options[selected as usize]),
                            color::Fg(color::Reset),
                        ).unwrap();
                    }
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }
        write!(
            stdout,
            "{}{}{}Would you like me to help with another task? y/n{}\n\r{}{}",
            clear::All,
            cursor::Goto(1,1),
            color::Fg(color::Cyan),
            color::Fg(color::Reset),
            cursor::Show,
            cursor::BlinkingUnderline,
        ).unwrap();
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Esc | Key::Char('q') | Key::Char('n') => {
                    running = false;
                    break;
                },
                Key::Char('y') => {
                    break;
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }
        first = false;
    }

    write!(
        stdout,
        "{}{}{}{}Closing Task Manager...",
        clear::All,
        cursor::Goto(1,1),
        cursor::Hide,
        color::Fg(color::Cyan),
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
        "...{}BYE{}",
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(500));
}

fn new_task(conn: &Connection) {
    if let Err(err) = conn.execute("INSERT INTO tasks ", []) {

    } else{

    }
}

fn retrieve_task(conn: &Connection) {
    
}

fn update_task(conn: &Connection) {
    
}

fn delete_task(conn: &Connection) {
    
}