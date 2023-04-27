use chrono::{DateTime, Local, NaiveDate, NaiveTime, NaiveDateTime, TimeZone};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::clear;
use termion::color;
use termion::style;
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use rusqlite::{Connection, params_from_iter};

use buddy_utils::format_name;

#[derive(Debug)]
struct Task {
    id: Option<u64>,
    title: String,
    description: String,
    due_date: Option<DateTime<Local>>,
    priority: u32,
    status: String,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
    completed_at: Option<DateTime<Local>>
}

impl Task {
    fn new(
        title: String,
        description: String,
        due_date: Option<DateTime<Local>>,
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

    fn from_db(
        id: u64,
        title: String,
        description: String,
        due_date: Option<DateTime<Local>>,
        priority: u32,
        status: String,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
        completed_at: Option<DateTime<Local>>
    ) -> Self {
        Self{
            id: Some(id),
            title,
            description,
            due_date,
            priority,
            status,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            completed_at
        }
    }
}

fn convert_datetime(value: String) -> Result<DateTime<Local>, ()>{
    if let Some(parsed_date) = DateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S%.f %:z").ok(){
        write!(stdout(), "Good").unwrap();
        stdout().flush().unwrap();
        let datetime = parsed_date.with_timezone(&Local);
        Ok(datetime)
    } else {
        write!(stdout(), "Broke").unwrap();
        stdout().flush().unwrap();
        Err(())
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
        if !running { break; }
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
    write!(
        stdout(),
        "{}{}{}Lets make a new task!",
        clear::All,
        cursor::Goto(1,1),
        color::Fg(color::Cyan)
    ).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(1200));
    write!(
        stdout(), 
        "{}{}{}{}", 
        clear::All, 
        cursor::Goto(1,1),
        cursor::Show,
        cursor::BlinkingUnderline
    ).unwrap();
    stdout().flush().unwrap();

    let mut title: String = String::new();
    let mut description: String = String::new();
    let mut due_date: Option<DateTime<Local>> = None;
    let mut priority: u32 = 5;
    let mut status: String = String::from("Created");
    let prompt_items: [&str; 5] = [
        "What is the TITLE of this new task?",
        "Please give a short DESCRIPTION of the task",
        "What is the DUE DATE associated with this task?\n\r  (Please format in mm-dd-yyyy)",
        "On a scale of 1-5, what is the PRIORITY of this task?",
        "Can you give a current STATUS? (an estimated completion percentage)",
    ];

    let mut idx = 0;
    while idx < prompt_items.len(){
        let field = prompt_items[idx];
        // Due Date precheck
        if idx == 2{
            write!(
                stdout(),
                "{}Is there a DUE DATE associated with this task? y/n\n\r {}> {}",
                color::Fg(color::Cyan),
                color::Fg(color::Red),
                color::Fg(color::Reset)
            ).unwrap();
            stdout().flush().unwrap();

            let mut date_associated = true;
            for key in stdin().keys(){
                match key.unwrap(){
                    Key::Esc => {
                        return;
                    },
                    Key::Char('y') => {
                        write!(stdout(), "y\n\r").unwrap();
                        break;
                    },
                    Key::Char('n') => {
                        date_associated = false;
                        due_date = None;
                        write!(stdout(), "n\n\r").unwrap();
                        break;
                    }
                    _ => {}
                }
            }

            // skip this prompt if not associated due date
            if !date_associated{ idx += 1; continue; }

        // Status precheck
        } else if idx == 4{
            write!(
                stdout(),
                "{}Have you started working on this task yet? y/n \n\r {}> {}",
                color::Fg(color::Cyan),
                color::Fg(color::Red),
                color::Fg(color::Reset),
            ).unwrap();
            stdout().flush().unwrap();

            let mut started = false;
            for key in stdin().keys(){
                match key.unwrap(){
                    Key::Esc => {
                        return;
                    },
                    Key::Char('y') => {
                        started = true;
                        write!(stdout(), "\n\r").unwrap();
                        break;
                    },
                    Key::Char('n') => {
                        write!(stdout(), "\n\r").unwrap();
                        break;
                    },
                    _ => {}
                }
            }
            if !started { idx += 1; continue; }
        }

        write!(
            stdout(),
            "{}{}\n\r{}> {}",
            color::Fg(color::Cyan),
            field,
            color::Fg(color::Red),
            color::Fg(color::Reset),
        ).unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        let mut valid_input = true;
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Esc => {
                    return;
                },
                Key::Delete | Key::Backspace => {
                    if input.len() > 0 {
                        input.pop();
                        write!(
                            stdout(),
                            "{}{}",
                            cursor::Left(1),
                            clear::AfterCursor,
                        ).unwrap();
                    }
                },
                Key::Char('\n') => {
                    match idx {
                        0 => {
                            title = input.clone();
                        },
                        1 => {
                            description = input.clone();
                        },
                        2 => {
                            if let Ok(valid) = validate_date_input(&input){
                                let date = NaiveDate::parse_from_str(&valid, "%m-%d-%Y").unwrap();
                                let time = NaiveTime::from_hms_opt(0,0,0).unwrap();
                                let datetime = NaiveDateTime::new(date, time);
                                due_date = Some(Local.from_local_datetime(&datetime).unwrap());
                            } else {
                                write!(
                                    stdout(),
                                    "\r{}{}Invalid Due Date value{}",
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                ).unwrap();
                                stdout().flush().unwrap();
                                sleep(Duration::from_millis(1500));
                                // Redo current prompt iteration
                                write!(
                                    stdout(),
                                    "{}{}",
                                    cursor::Goto(1, stdout().cursor_pos().unwrap().1 - 1),
                                    clear::AfterCursor,
                                ).unwrap();
                                stdout().flush().unwrap();
                                valid_input = false;
                                break;
                            }
                        },
                        3 => {
                            if let Ok(valid) = input.parse::<u32>(){
                                priority = valid;
                            } else{
                                write!(
                                    stdout(),
                                    "\r{}{}Invalid priority value{}",
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                ).unwrap();
                                stdout().flush().unwrap();
                                sleep(Duration::from_millis(1500));
                                // Redo current prompt iteration
                                write!(
                                    stdout(),
                                    "{}{}",
                                    cursor::Goto(1, stdout().cursor_pos().unwrap().1 - 1),
                                    clear::AfterCursor,
                                ).unwrap();
                                stdout().flush().unwrap();
                                valid_input = false;
                                break;
                            }
                        },
                        _ => {
                            if let Ok(valid) = input.parse::<u32>(){
                                match valid{
                                    0 => {
                                        break;
                                    },
                                    1..=35 => {
                                        status = String::from("Starting");
                                    },
                                    36..=70 => {
                                        status = String::from("Working");
                                    },
                                    71..=99 => {
                                        status = String::from("Finishing");
                                    },
                                    100 => {
                                        status = String::from("Completed");
                                    }
                                    _ => {
                                        write!(
                                            stdout(),
                                            "\r{}{}Invalid completion percentage value{}",
                                            clear::CurrentLine,
                                            color::Fg(color::Red),
                                            color::Fg(color::Reset),
                                        ).unwrap();
                                        stdout().flush().unwrap();
                                        sleep(Duration::from_millis(1500));
                                        // Redo current prompt iteration
                                        write!(
                                            stdout(),
                                            "{}{}",
                                            cursor::Goto(1, stdout().cursor_pos().unwrap().1 - 1),
                                            clear::AfterCursor,
                                        ).unwrap();
                                        stdout().flush().unwrap();
                                        valid_input = false;
                                        break;
                                    }
                                }
                            } else {
                                write!(
                                    stdout(),
                                    "\r{}{}Invalid completion percentage value{}",
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                ).unwrap();
                                stdout().flush().unwrap();
                                sleep(Duration::from_millis(1500));
                                // Redo current prompt iteration
                                write!(
                                    stdout(),
                                    "{}{}",
                                    cursor::Goto(1, stdout().cursor_pos().unwrap().1 - 1),
                                    clear::AfterCursor,
                                ).unwrap();
                                stdout().flush().unwrap();
                                valid_input = false;
                                break;
                            }
                        }
                    }
                    write!(stdout(), "\n\r").unwrap();
                    input.clear();
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
        }
        if valid_input{ idx += 1; }
    }

    let task = Task::new(title, description, due_date, priority, status);
    let mut query_field = String::from("INSERT INTO tasks (title, description, priority, status");
    let mut param_fields = String::from(" VALUES (?, ?, ?, ?");
    let mut parameters: Vec<String> = Vec::from([task.title, task.description, task.priority.to_string(), task.status]);

    if let Some(_) = task.due_date{
        query_field.push_str(", due_date");
        param_fields.push_str(", ?");
        parameters.push(task.due_date.map(|d| d.to_rfc3339()).unwrap());
    }
    if let Some(_) = task.completed_at{
        query_field.push_str(", completed_at");
        param_fields.push_str(", ?");
        parameters.push(task.completed_at.map(|d| d.to_rfc3339()).unwrap());
    }

    query_field.push(')');
    param_fields.push(')');
    query_field.push_str(&param_fields);
    let mut stmt = conn.prepare(&query_field).unwrap(); 
    if let Err(err) = stmt.execute(params_from_iter(parameters.iter())){
        write!(
            stdout(),
            "There has been an error with adding your task. {}",
            err,
        ).unwrap();
    } else{
        write!(
            stdout(),
            "Entry has been added successfully",
        ).unwrap();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(4000));
}

fn retrieve_task(conn: &Connection) {
    if let Ok(id) = display_all_tasks(conn, "retrieve"){
        let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?").unwrap();
        let mut row = stmt.query(&[&id]).unwrap();
        write!(
            stdout(),
            "{}Here is what I have found: \n\r",
            color::Fg(color::Green),
        ).unwrap();

        if let Some(entry) = row.next().unwrap(){
            let id: u64 = entry.get(0).unwrap();
            let title: String = entry.get(1).unwrap();
            let description: String = entry.get(2).unwrap();
            let due_date: Option<DateTime<Local>> = if entry.get::<usize, Option<String>>(3).unwrap().is_none(){
                None
            } else { 
                Some(convert_datetime(entry.get::<usize, Option<String>>(3).unwrap().unwrap()).unwrap())
            };
            let priority: u32 = entry.get(4).unwrap();
            let status: String = entry.get(5).unwrap();
            let created_at: String = entry.get(6).unwrap();
            let updated_at: String = entry.get(7).unwrap();
            // let created_at: DateTime<Local> = convert_datetime(entry.get(6).unwrap()).unwrap();
            // let updated_at: DateTime<Local> = convert_datetime(entry.get(7).unwrap()).unwrap();
            let completed_at: Option<DateTime<Local>> = if entry.get::<usize, Option<String>>(8).unwrap().is_none(){
                None
            } else{
                Some(convert_datetime(entry.get::<usize, Option<String>>(8).unwrap().unwrap()).unwrap())
            };
            // let task = Task::from_db(
            //     id,
            //     title,
            //     description,
            //     due_date,
            //     priority,
            //     status,
            //     created_at,
            //     updated_at,
            //     completed_at,
            // );
            write!(
                stdout(),
                "Id: {}\n\r
                Title: {}\n\r
                Description: {}\n\r
                Due Date: {:?}\n\r
                Priority: {}\n\r
                Status: {}\n\r
                Created At: {:?}\n\r
                Updated At: {:?}\n\r
                Completed At: {:?}\n\r",
                id,
                title,
                description,
                due_date,
                priority,
                status,
                created_at,
                updated_at,
                completed_at
            ).unwrap();
        } else{
            write!(
                stdout(),
                "{}There seems to be no results related to the Id: {}{}{}{}{}{}\n\r",
                color::Fg(color::Red),
                color::Fg(color::Magenta),
                style::Bold,
                style::Underline,
                id,
                style::Reset,
                color::Fg(color::Reset)
            ).unwrap();
        }

        write!(
            stdout(),
            "Please hit enter to continue."
        ).unwrap();
        stdout().flush().unwrap();
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Char('\n') | Key::Esc | Key::Char('q') => {
                    return;
                }
                _ => {}
            }
        }
    } else { return; }
}

fn update_task(conn: &Connection) {
    if let Ok(id) = display_all_tasks(conn, "update"){

    } else { return; }
}




fn delete_task(conn: &Connection) {
    if let Ok(id) = display_all_tasks(conn, "delete"){
        write!(
            stdout(),
            "",
            clear::All,
            cursor::Goto(1,1),
        ).unwrap();
        if let Err(err) = conn.execute("DELETE FROM tasks WHERE id = ?", &[&id]){
            write!(
                stdout(),
                "{}An error occured while deleting the task with id {}: {:?}{}",
                color::Fg(color::Red),
                id,
                err,
                color::Fg(color::Reset),
            ).unwrap();
        } else{
            write!(
                stdout(),
                "{}Successfuly deleted task with id {}{}",
                color::Fg(color::Green),
                id,
                color::Fg(color::Reset),
            ).unwrap();
        }
        stdout().flush().unwrap();
        sleep(Duration::from_millis(2500));
    } else { return; }

}

fn display_all_tasks(conn: &Connection, action: &str) -> Result<u32, ()>{
    // Prepare display retrieval
    let mut stmt = conn.prepare("SELECT id, title FROM tasks ORDER BY completed_at asc").unwrap();
    // Excecute query, map data, and collect
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0).unwrap(), row.get::<_, String>(1).unwrap()))
    }).unwrap();
    let items: Vec<_> = rows.map(|r| r.unwrap()).collect();

    write!(
        stdout(),
        "{}{}{}{}Please choose which entry you would like to {}.{}\n\r",
        cursor::Hide,
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Cyan),
        action,
        color::Fg(color::Reset)
    ).unwrap();
    stdout().flush().unwrap();

    let mut selected: u32 = 0;
    for (idx, entry) in items.iter().enumerate(){
        if idx == (selected as usize) {
            write!(
                stdout(),
                "{}  {}> {}) {}{}",
                cursor::Goto(1, idx as u16 + 2),
                color::Fg(color::Red),
                entry.0,
                entry.1.to_uppercase(),
                color::Fg(color::Reset)
            ).unwrap();
        } else{
            write!(
                stdout(),
                "{}    {}) {}{}",
                cursor::Goto(1, idx as u16 + 2),
                color::Fg(color::Reset),
                entry.0,
                entry.1,
            ).unwrap();
        }
    }
    stdout().flush().unwrap();

    for key in stdin().keys(){
        match key.unwrap(){
            Key::Up => {
                if selected > 0 {
                    write!(
                        stdout(),
                        "{}{}{}    {}) {}",
                        cursor::Goto(1, selected as u16 + 2),
                        clear::CurrentLine,
                        color::Fg(color::Reset),
                        &items[selected as usize].0,
                        &items[selected as usize].1,
                    ).unwrap();
                    selected -= 1;
                    write!(
                        stdout(),
                        "{}{}{}  > {}) {}{}",
                        cursor::Goto(1, selected as u16),
                        clear::CurrentLine,
                        color::Fg(color::Red),
                        &items[selected as usize].0,
                        &items[selected as usize].1,
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout().flush().unwrap();
                }
            },
            Key::Down => {
                if (selected as usize) < items.len() {
                    write!(
                        stdout(),
                        "{}{}{}    {}) {}",
                        cursor::Goto(1, selected as u16 + 2),
                        clear::CurrentLine,
                        color::Fg(color::Reset),
                        &items[selected as usize].0,
                        &items[selected as usize].1,
                    ).unwrap();
                    selected += 1;
                    write!(
                        stdout(),
                        "{}{}{}  > {}) {}{}",
                        cursor::Goto(1, selected as u16),
                        clear::CurrentLine,
                        color::Fg(color::Red),
                        &items[selected as usize].0,
                        &items[selected as usize].1,
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout().flush().unwrap();
                }
            },
            Key::Esc | Key::Char('q') => {
                return Err(());
            },
            Key::Char('\n') => {
                return Ok(items[selected as usize].0 as u32);
            },
            _ => {}
        }
    }
    Err(())
}

fn validate_date_input(date: &String) -> Result<String, ()>{
    Ok(date.to_string())
}