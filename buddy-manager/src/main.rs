use rusqlite::{Connection};
use std::io::{stdout, stdin, Write};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread::sleep;
use termion::clear;
use termion::color;
use termion::style;
use termion::cursor::{Goto, Hide, Show, BlinkingBlock};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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
    if let Ok(conn) = Connection::open("../../passwords_db.db3"){
        println!("Inside connection");
        // Check if the table already exists
        let table_exists: bool = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='passwords'")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .next()
            .is_some();
        println!("{}", table_exists);
        if !table_exists {
            // Create the table if it doesn't exist
            conn.execute(
                "CREATE TABLE passwords (
                    id INTEGER PRIMARY KEY,
                    site TEXT NOT NULL,
                    username TEXT NOT NULL,
                    email TEXT NOT NULL,
                    password TEXT NOT NULL
                )",
                [],
            ).unwrap();
        }
        conn.close().unwrap();
    }
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut running = true;
    let mut selected = 0;
    
    while running {
        let stdin = stdin();

        write!(
            stdout, 
            "{}{}{}{}{}{}Select a program to start: {}", 
            clear::All,
            Hide,
            Goto(1, 1),
            style::Bold,
            style::Underline,
            color::Fg(color::Green),
            style::Reset,
        ).unwrap();
        write!(
            stdout, 
            "{}{} enter / 'q'uit {}\n", 
            Goto(4, 2),
            color::Fg(color::Yellow),
            color::Fg(color::Reset)
        ).unwrap();
        stdout.flush().unwrap();

        // Print the list
        for (i, program) in BUDDY_PROGRAMS.iter().enumerate(){
            if selected == i {
                write!(
                    stdout,
                    "{}{}> {}{}{}{}",
                    Goto(1, (i+4) as u16),
                    color::Fg(color::Red),
                    style::Bold,
                    format_name(program).to_uppercase(),
                    color::Fg(color::Reset),
                    style::Reset
                ).unwrap();
            } else {
                write!(
                    stdout,
                    "{}{}{}",
                    Goto(1, (i+4) as u16),
                    color::Fg(color::Reset),
                    format_name(program),
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
                            format_name(BUDDY_PROGRAMS[selected])
                        ).unwrap();
                        selected -= 1;
                        write!(
                            stdout,
                            "{}{}{}> {}{}{}{}",
                            Goto(1, selected as u16 + 4),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            style::Bold,
                            format_name(BUDDY_PROGRAMS[selected]).to_uppercase(),
                            color::Fg(color::Reset),

                            style::Reset
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
                            format_name(BUDDY_PROGRAMS[selected]),
                        ).unwrap();
                        selected += 1;
                        write!(
                            stdout,
                            "{}{}{}> {}{}{}{}",
                            Goto(1, selected as u16 + 4),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            style::Bold,
                            format_name(BUDDY_PROGRAMS[selected]).to_uppercase(),
                            color::Fg(color::Reset),
                            style::Reset
                        ).unwrap();
                    }
                },
                Key::Char('\n') => {
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
                        sleep(Duration::from_millis(100));
                    }
                    write!(
                        stdout, 
                        "..Selected{}",
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout.flush().unwrap();
                    sleep(Duration::from_secs(1));

                    write!(
                        stdout,
                        "{}{}{}{}You have chosesen: {}{}{}\n\r",
                        clear::All,
                        Goto(1, 1),
                        color::Fg(color::Green),
                        style::Underline,
                        BUDDY_PROGRAMS[selected].to_uppercase(),
                        Goto(1, 2),
                        style::NoUnderline,
                    ).unwrap();
                    stdout.flush().unwrap();
                    sleep(Duration::from_millis(500));
                    break;
                },
                Key::Char('q') | Key::Esc => {
                    write!(
                        stdout,
                        "{}{}{}Leaving your buddy behind....",
                        clear::All,
                        Goto(1,1),
                        color::Fg(color::Red),
                    ).unwrap();
                    stdout.flush().unwrap();
                    for _i in 0..5{
                        write!(
                            stdout, 
                            "...",
                        ).unwrap();
                        stdout.flush().unwrap();
                        sleep(Duration::from_millis(200));
                    }
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
        sleep(Duration::from_millis(1500));
        
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
        sleep(Duration::from_secs(3));
        println!(
            "{}{} Exiting {}. Status: {}",
            clear::All, 
            Goto(1,1),
            BUDDY_PROGRAMS[selected], 
            exit_status);
        sleep(Duration::from_millis(500));
    }

    write!(
        stdout,
        "...Goodbye {}{}{}BUDDY.{}{}",
        color::Fg(color::Green),
        style::Bold,
        style::Underline,
        style::Reset,
        color::Fg(color::Reset),
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(1));

    drop(&mut stdout);
    write!(
        stdout,
        "{}{}{}\r",
        clear::All,
        BlinkingBlock,
        Show
    ).unwrap();
    stdout.flush().unwrap();

    return;
}

fn format_name(program: &str) -> String{
    let mut title = String::new();
    for (idx, word) in program.split('-').enumerate(){
        title.push_str(&word[0..1].to_uppercase());
        title.push_str(&word[1..]);
        title.push(' ');
    }
    title
}