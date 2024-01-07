use std::io::{Write, Stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::{style, cursor::Goto, color};

use crate::utils::{self, terminal};

pub fn enter(mut stdout: &Stdout, message: &str){
    terminal::clear_terminal(stdout);
    terminal::cursor_display(stdout, false);
    write!(
        stdout,
        "{}",
        terminal::buddy_text( message)
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(600));
}

// Print the program list to Terminal
pub fn print_programs(mut stdout: &Stdout, selected: usize, options: &[&str], offset: usize){
    for (i, program) in options.iter().enumerate(){
        write!(
            stdout,
            "{}{}\r",
            Goto(1, (i + offset) as u16),
            " ".repeat(25),
        ).unwrap();
        if selected == i {
            write!(
                stdout,
                "{}{}{}",
                style::Bold,
                terminal::error_text(
                    &("> ".to_owned() + &utils::format_name(program))
                ),
                style::Reset
            ).unwrap();
        } else {
            write!(
                stdout,
                "{}",
                terminal::plain_text(&utils::format_name(program)),
            ).unwrap();
        }
    }
    stdout.flush().unwrap();
}


pub fn program_selection(mut stdout: &Stdout, selected: usize, options: &[&str], offset: usize){
    write!(
        stdout, 
        "{}{}",
        Goto(1, (selected + offset) as u16),
        terminal::success_text(
            &(utils::format_name(options[selected]) + &".".repeat(2))
        ),
    ).unwrap();
    stdout.flush().unwrap();
    for _i in 0..4{
        write!(
            stdout,
            "{}",
            terminal::success_text(".."),
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(60));
    }
    write!(
        stdout, 
        "{}",
        terminal::success_text("..Selected"),
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(400));

    terminal::clear_terminal(stdout);
    write!(
        stdout,
        "{}{}{} {}{}{}{}\n\r",
        style::Underline,
        terminal::success_text("Opening:"),
        style::Reset,
        style::Bold,
        style::Italic,
        utils::format_name(options[selected]),
        style::Reset,
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(400));
}

pub fn exit(mut stdout: &Stdout, start_phrase: &str, end_phrase: &str){
    terminal::clear_terminal(stdout);
    terminal::cursor_display(stdout, false);
    write!(
        stdout,
        "{}",
        terminal::error_text( start_phrase)
    ).unwrap();
    stdout.flush().unwrap();
    for _ in 0..5{
        write!(
            stdout,
            "{}",
            terminal::error_text("..."),
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(50));
    }
    write!(
        stdout,
        "{}{}",
        terminal::buddy_text(end_phrase),
        style::Reset
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(600));
}
