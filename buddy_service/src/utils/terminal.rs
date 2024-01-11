use std::io::{Write, Stdout};
use termion::{color, cursor, clear };


fn terminal_mesage(message: &str, color: impl color::Color) -> String{
    return format!(
        "{}{}{}",
        color::Fg(color),
        message,
        color::Fg(color::Reset)
    )
}

pub fn plain_text(message: &str) -> String {
    return terminal_mesage(message, color::Reset);
}

pub fn buddy_text(message: &str) -> String {
    return terminal_mesage(message,color::Cyan);
}

pub fn success_text(message: &str) -> String {
    return terminal_mesage(message,color::Green);
}

pub fn info_text(message: &str) -> String {
    return terminal_mesage(message,color::Yellow);
}

pub fn warning_text(message: &str) -> String {
    return terminal_mesage(message, color::Red);
}

pub fn error_text(message: &str) -> String {
    return terminal_mesage(message, color::Magenta);
}

pub fn clear_terminal(mut stdout: &Stdout){
    write!(
        stdout, 
        "{}{}", 
        clear::All, 
        cursor::Goto(1, 1)
    ).unwrap();
    stdout.flush().unwrap();
}

pub fn clear_line(mut stdout: &Stdout){
    write!(
        stdout,
        "\r{}",
        clear::CurrentLine,
    ).unwrap();
    stdout.flush().unwrap();
}

pub fn cursor_display(mut stdout: &Stdout, show_cursor: bool){
    if show_cursor{
        write!(
            stdout, 
            "{}", 
            cursor::Show
        ).unwrap();
    } else {
        write!(
            stdout, 
            "{}", 
            cursor::Hide
        ).unwrap();
    }
    stdout.flush().unwrap();
}