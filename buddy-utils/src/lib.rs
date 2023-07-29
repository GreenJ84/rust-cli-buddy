use std::io::{Write, Stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::{color, cursor, clear, style };

pub fn main(){}

pub fn format_name(program: &str) -> String{
    let mut title = String::new();
    for (_, word) in program.split('-').enumerate(){
        if word == "ai"{
            title.push_str(&word.to_uppercase());
            title.push(' ');
            continue;
        }
        title.push_str(&word[0..1].to_uppercase());
        title.push_str(&word[1..]);
        title.push(' ');
    }
    title.pop();
    title
}

pub fn application_entry(mut stdout: &Stdout, message: &str){
    write!(
        stdout,
        "{}{}{}{}{}{}",
        clear::All,
        cursor::Hide,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        message,
        color::Fg(color::Reset)
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(800));
}

pub fn application_close(mut stdout: &Stdout, start_phrase: &str, end_phrase: &str){
    write!(
        stdout,
        "{}{}{}{}{}..",
        clear::All,
        cursor::Hide,
        cursor::Goto(1, 1),
        color::Fg(color::Red),
        start_phrase,
    ).unwrap();
    for _ in 0..5{
        write!(
            stdout,
            "...",
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    write!(
        stdout,
        "{}{}{}{}",
        color::Fg(color::Green),
        end_phrase,
        color::Fg(color::Reset),
        style::Reset
    ).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_millis(1000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_name_test() {
        assert_eq!(format_name(&"ai-assistant"), String::from("AI Assistant"));
        assert_eq!(format_name(&"calculator"), String::from("Calculator"));
        assert_eq!(format_name(&"development-timer"), String::from("Development Timer"));
        assert_eq!(format_name(&"file-organizer"), String::from("File Organizer"));
        assert_eq!(format_name(&"password-generator"), String::from("Password Generator"));
        assert_eq!(format_name(&"password-manager"), String::from("Password Manager"));
        assert_eq!(format_name(&"task-manager"), String::from("Task Manager"));
        assert_eq!(format_name(&"word-analyzer"), String::from("Word analyzer"));
    }
}
