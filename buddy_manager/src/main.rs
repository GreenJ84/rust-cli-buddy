use dotenv::dotenv;
use std::io::{stdin, stdout, Write};
use std::{thread::sleep, time::Duration};
use termion::{cursor::Goto, event::Key, input::TermRead, raw::IntoRawMode, style};

use buddy_service::{
    database,
    utils::{self, application, terminal},
};

const BUDDY_PROGRAMS: [&str; 8] = [
    "ai-assistant",
    "calculator",        //done, needs Polish
    "development-timer", // started
    "file-organizer",
    "password-manager",   // done, needs Polish
    "password-generator", // done, needs Polish
    "task-manager",
    "word analyzer",
];

fn main() {
    dotenv().ok();
    // Enter raw control of terminal
    let mut stdout = stdout()
        .into_raw_mode()
        .expect("Buddy can't take control of the terminal");
    terminal::clear_terminal(&stdout);

    // SQLite DB setup for the application
    if let Err(e) = database::database_establishment() {
        application::error_confirmation(
            &mut stdout,
            &format!("Failed to establish database connection: \n\r\t{:?}\n\rSome applications will be affected", e),
        );
    };
    application::enter(&stdout, "Welcome back Buddy!");

    // Program Manager selection loop
    let mut running = true;
    let mut selected = 0;
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
        )
        .unwrap();
        stdout.flush().unwrap();

        // Print the application list to Terminal
        application::print_programs(&stdout, selected, BUDDY_PROGRAMS.as_slice(), 4);

        // Handle User Selection Input
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Up => {
                    if selected > 0 {
                        selected -= 1;
                        application::print_programs(
                            &stdout,
                            selected,
                            BUDDY_PROGRAMS.as_slice(),
                            4,
                        );
                    }
                }
                Key::Down => {
                    if selected < BUDDY_PROGRAMS.len() - 1 {
                        selected += 1;
                        application::print_programs(
                            &stdout,
                            selected,
                            BUDDY_PROGRAMS.as_slice(),
                            4,
                        );
                    }
                }
                Key::Char('\n') => {
                    application::program_selection(&stdout, selected, BUDDY_PROGRAMS.as_slice(), 4);
                    break;
                }
                Key::Char('q') | Key::Esc => {
                    running = false;
                    break;
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
        if running {
            // sub-Program opening
            write!(
                stdout,
                "Starting {}",
                terminal::buddy_text(&utils::format_name(BUDDY_PROGRAMS[selected]))
            )
            .unwrap();
            stdout.flush().unwrap();
            sleep(Duration::from_millis(300));
            terminal::clear_terminal(&stdout);
    
            buddy_manager::spawn_program(BUDDY_PROGRAMS[selected]);
        }
    }

    application::exit(&stdout, "Leaving your buddy behind...", "WHY!?!?!");
    
    terminal::clear_terminal(&stdout);
    terminal::cursor_display(&stdout, true);

    // Drop output control
    drop(stdout);
    return;
}
