use std::io::{stdout, stdin, Write, Stdout};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread::sleep;
use termion::clear;
use termion::color;
use termion::style;
use termion::cursor::{Goto, Show, BlinkingBlock};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use rusqlite::Connection;

use buddy_utils::{format_name, application_entry, application_close};

const BUDDY_PROGRAMS: [&str; 8] = [
    "ai-assistant",
    "calculator", //done, needs Polish
    "development-timer", // started
    "file-organizer",
    "password-manager", // done, needs Polish
    "password-generator", // done, needs Polish
    "task-manager", 
    "word analyzer",
];

fn main() {
    database_establishment();
    clear_termianl();
    let mut stdout = stdout().into_raw_mode().unwrap();
    application_entry(&stdout, "Welcome back Buddy!");

    let mut running = true;
    let mut selected = 0;
    while running {
        let stdin = stdin();

        // Print Section title to Terminal
        write!(
            stdout, 
            "{}{}{}{}{}Select a program to start: {}{}{} enter / 'q'uit {}\n", 
            clear::All,
            Goto(1, 1),
            style::Bold,
            style::Underline,
            color::Fg(color::Green),
            style::Reset,
            Goto(4, 2),
            color::Fg(color::Yellow),
            color::Fg(color::Reset)
        ).unwrap();
        stdout.flush().unwrap();

        // Print the list to Terminal
        print_programs(&stdout, selected);

        // Handle User Input
        for c in stdin.keys() {
            match c.unwrap(){
                Key::Up => {
                    if selected > 0{
                        selected -= 1;
                        print_programs(&stdout, selected);
                    }
                },
                Key::Down => {
                    if selected < BUDDY_PROGRAMS.len() - 1{
                        selected += 1;
                        print_programs(&stdout, selected);
                    }
                },
                Key::Char('\n') => {
                    program_selection(&stdout, selected);
                    break;
                },
                Key::Char('q') | Key::Esc => {
                    application_close(&stdout, "Leaving your buddy behind...");
                    running = false;
                    break;
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }
        if !running { break; }

        write!(
            stdout,
            "Starting {}{}", 
            BUDDY_PROGRAMS[selected].to_uppercase(),
            style::Reset
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(300));

        clear_termianl();
        let mut spawn = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(BUDDY_PROGRAMS[selected])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn application");

        let exit_status = spawn.wait().expect(&format!("Failed to wait for {} program.", BUDDY_PROGRAMS[selected]));
        sleep(Duration::from_millis(500));
        if !exit_status.success() {
            eprintln!("Sub Application exited with an error.");
        }

        println!(
            "{}{} Exiting {}. Status: {}",
            clear::All, 
            Goto(1,1),
            BUDDY_PROGRAMS[selected], 
            exit_status);
        sleep(Duration::from_millis(500));
    }

    // Hurt their emotions
    write!(
        stdout,
        "...{}{}{}BUDDY.{}{}",
        color::Fg(color::Green),
        style::Bold,
        style::Underline,
        style::Reset,
        color::Fg(color::Reset),
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(1));

    // Clear and reset all of terminal settings
    write!(
        stdout,
        "{}{}{}{}{}\r",
        clear::All,
        style::Reset,
        color::Fg(color::Reset),
        Show,
        BlinkingBlock,
    ).unwrap();
    stdout.flush().unwrap();

    // Drop output control
    drop(stdout);
    return;
}

fn database_establishment(){
    match Connection::open("../../passwords_db.db3"){
        Ok(conn) => {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS passwords (
                    id INTEGER PRIMARY KEY,
                    site TEXT NOT NULL,
                    username TEXT NOT NULL,
                    email TEXT NOT NULL,
                    password TEXT NOT NULL
                )",
                [],
            ).unwrap();
            conn.close().unwrap();
        }
        Err(e) => {
            println!("Error connecting to the passwords database: {}", e);
        }
    }

    match Connection::open("../../tasks_db.db3"){
        Ok(conn) => {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    description TEXT NOT NULL,
                    due_date TEXT,
                    priority INTEGER NOT NULL,
                    status TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    completed_at TEXT
                )",
                []
            ).unwrap();
            conn.close().unwrap();
        },
        Err(err) => {
            eprintln!("Error connecting to the tasks database: {}", err);
        }
    }
}


// Print the program list to Terminal
fn print_programs(mut stdout: &Stdout, selected: usize){
    for (i, program) in BUDDY_PROGRAMS.iter().enumerate(){
        write!(
            stdout,
            "{}{}\r",
            Goto(1, (i+4) as u16),
            " ".repeat(25),
        ).unwrap();
        if selected == i {
            write!(
                stdout,
                "{}> {}{}{}{}",
                color::Fg(color::Red),
                style::Bold,
                format_name(program).to_uppercase(),
                color::Fg(color::Reset),
                style::Reset
            ).unwrap();
        } else {
            write!(
                stdout,
                "{}{}",
                color::Fg(color::Reset),
                format_name(program),
            ).unwrap();
        }
    }
    stdout.flush().unwrap();
}

fn program_selection(mut stdout: &Stdout, selected: usize){
    write!(
        stdout, 
        "{}{}..",
        Goto(BUDDY_PROGRAMS[selected].len() as u16 + 3, selected as u16 + 4),
        color::Fg(color::Red),
    ).unwrap();
    stdout.flush().unwrap();

    for _i in 0..5{
        write!(
            stdout, 
            "..",
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(80));
    }

    write!(
        stdout, 
        "..Selected{}",
        color::Fg(color::Reset),
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(400));

    write!(
        stdout,
        "{}{}{}{}You have chosesen: {}{}{}\n\r",
        clear::All,
        Goto(1, 1),
        color::Fg(color::Green),
        style::Underline,
        format_name(BUDDY_PROGRAMS[selected]).to_uppercase(),
        Goto(1, 2),
        style::Reset,
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(400));
}

fn clear_termianl(){
    // Unix
    if cfg!(unix) {
        // Unix-based system (Linux, macOS, etc.)
        let status = Command::new("clear").status().unwrap();
        if !status.success() {
            eprintln!("Failed to clear the terminal.");
        }
    } else if cfg!(windows) {
        // Windows
        let status = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
        if !status.success() {
            eprintln!("Failed to clear the terminal.");
        }
    } else {
        eprintln!("Unsupported operating system to clear.");
    }
}