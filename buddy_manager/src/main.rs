use std::io::{stdout, stdin, Write};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread::sleep;
use termion::style;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


use buddy_service::{database, utils::{self, terminal, application}};

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
    // Enter raw control of terminal
    let mut stdout = stdout().into_raw_mode().unwrap();
    // Clear all previous terminal dialogue
    terminal::clear_terminal(&stdout);
    // SQLite DB setup for the application
    database::database_establishment();
    // Buddy Manager application entrance
    application::enter(&stdout, "Welcome back Buddy!");

    let mut running = true;
    let mut selected = 0;
    // Program Manager selection loop
    while running {
        let stdin = stdin();

        terminal::clear_terminal(&stdout);
        write!(
            stdout, 
            "{}{}{}{}{}{}\n", 
            style::Bold,
            style::Underline,
            terminal::buddy_text("Select a program to start:"),
            Goto(3, 2),
            terminal::info_text("(enter/'q'uit)"),
            style::Reset
        ).unwrap();
        stdout.flush().unwrap();

        // Print the list to Terminal
        application::print_programs(&stdout, selected, BUDDY_PROGRAMS.as_slice(), 4);

        // Handle User Input
        for c in stdin.keys() {
            match c.unwrap(){
                Key::Up => {
                    if selected > 0{
                        selected -= 1;
                        application::print_programs(&stdout, selected, BUDDY_PROGRAMS.as_slice(), 4);
                    }
                },
                Key::Down => {
                    if selected < BUDDY_PROGRAMS.len() - 1{
                        selected += 1;
                        application::print_programs(&stdout, selected, BUDDY_PROGRAMS.as_slice(), 4);
                    }
                },
                Key::Char('\n') => {
                    application::program_selection(&stdout, selected, BUDDY_PROGRAMS.as_slice(), 4);
                    break;
                },
                Key::Char('q') | Key::Esc => {
                    application::exit(&stdout, "Leaving your buddy behind...", "WHY!?!?!");
                    running = false;
                    break;
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }
        if !running { break; }

        // Program opening dialogue
        write!(
            stdout,
            "Starting {}", 
            terminal::buddy_text(&utils::format_name(BUDDY_PROGRAMS[selected]))
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(300));
        terminal::clear_terminal(&stdout);

        spawn_program(BUDDY_PROGRAMS[selected]);
    }
    terminal::clear_terminal(&stdout);
    terminal::cursor_display(&stdout, true);

    // Drop output control
    drop(stdout);
    return;
}


fn spawn_program(program: &str) {
    // Create a child process
    let mut command = Command::new("cargo");
    
    command
        .arg("run")
        .arg("-p")
        .arg(program)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Spawn child process
    if let Ok(mut spawn) = 
        command.spawn() {
            let exit_status = spawn.wait().expect(&format!("Failed to run {} program.", program));
            if !exit_status.success() {
                eprint!("Sub Application exited with an error.");
            } else {
                println!(
                    "\n\r{}",
                    terminal::info_text(
                        &format!("Exiting {}. Status: {}",
                        program, 
                        exit_status)
                    )
                );
            }
    } else {
        eprint!("Program spawning command failed to initialize.");
    }

    sleep(Duration::from_millis(500));
}