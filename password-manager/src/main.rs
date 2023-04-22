use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, stdin, Write};
use rusqlite::Connection;
use termion::clear;
use termion::color;
use termion::style;
use termion::cursor;
use termion::input::TermRead;
use termion::event::{Key};

#[derive(Debug)]
struct User {
    id: u32,
    site: String,
    username: String,
    email: String,
    password: String
}
// Register cli user, add, manage, delete passwords
fn main() {
    let mut stdout = stdout();
    let mut conn = Connection::open("../../passwords_db.db3").unwrap();

    write!(
        stdout,
        "{}{}{}Welcome to your password manager!{}\n\n\r",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    ).unwrap();
    sleep(Duration::from_millis(600));
    let options: [&str; 4] = ["'N'ew password", "'R'etrieve password", "'U'pdate password", "'D'elete password"];
    
    let mut running = true;
    let mut first = true;
    while running {
        let stdin = stdin();
        write!(
            stdout,
            "{}{}{}What can I help with {}?{}\n\r",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Cyan),
            if first { "today "} else { "next" },
            color::Fg(color::Reset)
        ).unwrap();
        stdout.flush().unwrap();
        
        let mut selected: u16 = 0;
        for (idx, option) in options.iter().enumerate(){
            if idx == usize::from(selected) {
                write!(
                    stdout,
                    "{}  > {}{}{}{}{}",
                    cursor::Goto(1, (idx+2) as u16),
                    color::Fg(color::Red),
                    style::Bold,
                    option.to_uppercase(),
                    color::Fg(color::Reset),
                    style::Reset
                ).unwrap();
            } else{
                write!(
                    stdout,
                    "{}{}    {}",
                    cursor::Goto(1, (idx+2) as u16),
                    color::Fg(color::Reset),
                    option,
                ).unwrap();
            }
            stdout.flush().unwrap()
        }
        
        for key in stdin.keys(){
            match key.unwrap(){
                Key::Esc | Key::Char('q')=> {
                    running = false;
                    break;
                },
                Key::Up => {
                    if selected > 0 {
                        write!(
                            stdout,
                            "{}{}{}    {}",
                            cursor::Goto(1, selected+2),
                            clear::CurrentLine,
                            color::Fg(color::Reset),
                            options[selected as usize],
                        ).unwrap();
                        selected -= 1;
                        write!(
                            stdout,
                            "{}{}{}  > {}{}",
                            cursor::Goto(1, selected+2),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            options[selected as usize],
                            color::Fg(color::Reset)
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
                },
                Key::Down => {
                    if usize::from(selected) < options.len(){
                        write!(
                            stdout,
                            "{}{}{}    {}",
                            cursor::Goto(1, selected+2),
                            clear::CurrentLine,
                            color::Fg(color::Reset),
                            options[selected as usize],
                        ).unwrap();
                        selected += 1;
                        write!(
                            stdout,
                            "{}{}{}  > {}{}",
                            cursor::Goto(1, selected+2),
                            clear::CurrentLine,
                            color::Fg(color::Red),
                            options[selected as usize],
                            color::Fg(color::Reset)
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
                },
                Key::Char('\n') => {
                    match selected {
                        0 => {
                            new_pass(&conn);
                        },
                        1 => {
                            retrieve_pass(&conn);
                        },
                        2 => {
                            update_pass(&conn);
                        },
                        _ => {
                            delete_pass(&conn);
                        }
                    }
                    break;
                },
                Key::Char('N') | Key::Char('n') => {
                    new_pass(&conn);
                    break;
                },
                Key::Char('R') | Key::Char('r') => {
                    retrieve_pass(&conn);
                    break;
                }, 
                Key::Char('U') | Key::Char('u') => {
                    update_pass(&conn);
                    break;
                },
                Key::Char('D') | Key::Char('d')=> {
                    delete_pass(&conn);
                    break;
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
        first = false;
        if !running { break; }
    }

    write!(
        stdout,
        "{}{}{}Exiting password manager...",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Red)
    ).unwrap();
    for _ in 0..5{
        write!(
            stdout,
            "..."
        ).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(200));
    }
    write!(
        stdout,
        "BYE"
    ).unwrap();
    stdout.flush().unwrap();


}
fn retrieve_pass(conn: &Connection){
    let mut password_site: String = String::new();
    if let Some(site) = get_pass_site("retrieve"){
        password_site = site;
    } else { return; }

    let mut stmt = conn.prepare("SELECT * FROM passwords WHERE site = ?").unwrap();
    let mut rows = stmt.query(&[&password_site]).unwrap();
    write!(
        stdout(),
        "{}Here is what I have found:",
        color::Fg(color::Green),
    ).unwrap();

    let mut entry_found = false;
    while let Some(row) = rows.next().unwrap() {
        entry_found = true;

        let username: String = row.get(2).unwrap();
        let email: String = row.get(3).unwrap();
        let password: String = row.get(4).unwrap();
        write!(
            stdout(),
            "\n\rSite: {}{}{}{}{}{}\n\rUsername: {}{}{}{}{}{}\n\rEmail: {}{}{}{}{}{}\n\rPassword: {}{}{}{}{}{} \n\n\r",
            color::Fg(color::Cyan),
            style::Bold,
            style::Underline,
            password_site,
            style::Reset,
            color::Fg(color::Green),
            color::Fg(color::Cyan),
            style::Bold,
            style::Underline,
            username,
            style::Reset,
            color::Fg(color::Green),
            color::Fg(color::Cyan),
            style::Bold,
            style::Underline,
            email,
            style::Reset,
            color::Fg(color::Green),
            color::Fg(color::Cyan),
            style::Bold,
            style::Underline,
            password,
            style::Reset,
            color::Fg(color::Reset)
        ).unwrap()
    } 
    if !entry_found {
        write!(
            stdout(),
            "{}There seems to be no results related to the site {}{}{}{}{}{}",
            color::Fg(color::Red),
            color::Fg(color::Magenta),
            style::Bold,
            style::Underline,
            password_site,
            style::Reset,
            color::Fg(color::Reset)
        ).unwrap();
    }
    stdout().flush().unwrap();
    for key in stdin().keys(){
        match key.unwrap(){
            Key::Esc | Key::Char('\n') | Key::Char(' ') => {
                break;
            },
            _ => {}
        }
    }
}

fn update_pass(conn: &Connection){
    let mut password_site: String = String::new();
    if let Some(site) = get_pass_site("update"){
        password_site = site;
    } else { return; }

    let info: [&str; 3] = [
            "What is the USERNAME that will be associated with this password?",
            "What is the EMAIL associated with the password or site? (or N/A)",
            "What is your new PASSWORD?"
    ];
    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    for (idx, phrase) in info.iter().enumerate(){
        write!(
            stdout(),
            "{}{}{}\n\r",
            color::Fg(color::Cyan),
            phrase,
            color::Fg(color::Reset)
        ).unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Esc => {
                    return;
                },
                Key::Delete | Key::Backspace => {
                    if input.len() > 0 as usize {
                        input.pop();
                        write!(
                            stdout(),
                            "{}{}",
                            cursor::Left(1),
                            clear::AfterCursor,
                        ).unwrap();
                        stdout().flush().unwrap();}
                },
                Key::Char('\n') => {
                    match idx {
                        0 => {
                            username = input.to_owned();
                        },
                        1 => {
                            email = input.to_owned();
                        }, 
                        _ => {
                            password = input.to_owned();
                        }
                    }
                    input = String::new();
                    write!(stdout(), "\n\r").unwrap();
                    stdout().flush().unwrap();
                    break;
                },
                Key::Char(c) => {
                    input.push(c);
                    write!(stdout(), "{}", c).unwrap();
                },
                _ => {}
            }
            stdout().flush().unwrap();
            if idx == 2 {
                match input.len() {
                    0 => {},
                    1 | 2 | 3 | 4 | 5 | 6 => {
                        write!(
                            stdout(),
                            "\r{}{}{}",
                            clear::AfterCursor,
                            color::Fg(color::Red),
                            input
                        ).unwrap();
                        stdout().flush().unwrap();
                    },
                    7 | 8 | 9 | 10 | 11 => {
                        write!(
                            stdout(),
                            "\r{}{}{}",
                            clear::AfterCursor,
                            color::Fg(color::Yellow),
                            input
                        ).unwrap();
                        stdout().flush().unwrap();
                    },
                    _ => {
                        write!(
                            stdout(),
                            "\r{}{}{}",
                            clear::AfterCursor,
                            color::Fg(color::Green),
                            input
                        ).unwrap();
                        stdout().flush().unwrap();
                    }
                }
                stdout().flush().unwrap();
            }
        }
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    if let Err(err) = conn.execute(
        "UPDATE passwords SET password = ? WHERE site = ?",
        [ &password_site]
    ){

    }
}

fn delete_pass(conn: &Connection){
    let mut password_site: String = String::new();
    if let Some(site) = get_pass_site("delete"){
        password_site = site;
    } else { return; }
    
    if let Err(err) = conn.execute("DELETE FROM passwords WHERE site = ?", [&password_site]){
        write!(
            stdout(), 
            "{}Couldn't delete password for {}{}{}{}{}{}: {:?}{}", 
            color::Fg(color::Red),
            color::Fg(color::Magenta),
            style::Bold,
            style::Underline,
            password_site,
            style::Reset,
            color::Fg(color::Red),
            err,
            color::Fg(color::Reset)
        ).unwrap();
    } else {
        write!(
            stdout(), 
            "{}Deleted password for {}{}{}{}{}{}", 
            color::Fg(color::Green),
            color::Fg(color::Cyan),
            style::Bold,
            style::Underline,
            password_site,
            style::Reset,
            color::Fg(color::Reset)
        ).unwrap();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
}

fn get_pass_site(operation: &str) -> Option<String>{
    write!(
        stdout(),
        "{}{}{}For what site would you like to {} your password?{}\n\r",
        clear::All,
        cursor::Goto(1,1),
        color::Fg(color::Cyan),
        operation,
        color::Fg(color::Reset)
    ).unwrap();
    stdout().flush().unwrap();

    let mut input = String::new();
    for key in stdin().keys(){
        match key.unwrap() {
            Key::Esc => {
                return None;
            }
            Key::Char('\n') => {
                write!(stdout(), "\n\r").unwrap();
                return Some(input);
            },
            Key::Char(c) => {
                input.push(c);
                write!(stdout(), "{}", c).unwrap();
            },
            Key::Delete => {
                input.pop();
                write!(
                    stdout(),
                    "{}{}",
                    cursor::Left(1),
                    clear::AfterCursor,
                ).unwrap();
                stdout().flush().unwrap();
            },
            _ => {}
        }
        stdout().flush().unwrap();
    }
    return None;
}

fn new_pass(conn: &Connection){
    write!(
        stdout(),
        "{}{}",
        clear::All,
        cursor::Goto(1, 1),
    ).unwrap();

    let info: [&str; 4] = [
            "What SITE will the password be for?", 
            "Is there a USERNAME associated with the password or site?",
            "Is there am EMAIL associated with the password or site?",
            "What is the PASSWORD?"
        ];

    let mut site = String::new();
    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    for (idx, phrase) in info.iter().enumerate(){
        write!(
            stdout(),
            "{}{}{}\n\r",
            color::Fg(color::Cyan),
            phrase,
            color::Fg(color::Reset)
        ).unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Esc => {
                    return;
                },
                Key::Delete | Key::Backspace => {
                    if input.len() > 0 as usize {
                        input.pop();
                        write!(
                            stdout(),
                            "{}{}",
                            cursor::Left(1),
                            clear::AfterCursor,
                        ).unwrap();
                        stdout().flush().unwrap();}
                },
                Key::Char('\n') => {
                    match idx {
                        0 => {
                            site = input.to_owned();
                        },
                        1 => {
                            username = input.to_owned();
                        },
                        2 => {
                            email = input.to_owned();
                        }, 
                        _ => {
                            password = input.to_owned();
                        }
                    }
                    input = String::new();
                    write!(stdout(), "\n\r").unwrap();
                    stdout().flush().unwrap();
                    break;
                },
                Key::Char(c) => {
                    input.push(c);
                    write!(stdout(), "{}", c).unwrap();
                },
                _ => {}
            }
            stdout().flush().unwrap();
            if idx == 3 {
                match input.len() {
                    0 => {},
                    1 | 2 | 3 | 4 | 5 | 6 => {
                        write!(
                            stdout(),
                            "\r{}{}{}",
                            clear::AfterCursor,
                            color::Fg(color::Red),
                            input
                        ).unwrap();
                        stdout().flush().unwrap();
                    },
                    7 | 8 | 9 | 10 | 11 => {
                        write!(
                            stdout(),
                            "\r{}{}{}",
                            clear::AfterCursor,
                            color::Fg(color::Yellow),
                            input
                        ).unwrap();
                        stdout().flush().unwrap();
                    },
                    _ => {
                        write!(
                            stdout(),
                            "\r{}{}{}",
                            clear::AfterCursor,
                            color::Fg(color::Green),
                            input
                        ).unwrap();
                        stdout().flush().unwrap();
                    }
                }
                stdout().flush().unwrap();
            }
        }
    }
    if let Err(err) = conn.execute(
        "INSERT INTO passwords ( site, username, email, password) VALUES (?, ?, ?, ?)",
        [&site, &username, &email, &password]
    ){
        write!(
            stdout(),
            "{}There seems to be an error saving you info: {}{}{:?}{}{}",
            color::Fg(color::Red),
            color::Fg(color::Magenta),
            style::Bold,
            err,
            style::Reset,
            color::Fg(color::Reset)
        ).unwrap();
    } else{
        write!(
            stdout(),
            "\n\r{}Saved your password information for {}{}{}{}{}{}",
            color::Fg(color::Green),
            color::Fg(color::Cyan),
            style::Bold,
            style::Underline,
            site,
            style::Reset,
            color::Fg(color::Reset)
        ).unwrap();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(1500));
}


