mod task;

use chrono::{DateTime, Local, NaiveDate, NaiveTime, NaiveDateTime, TimeZone};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::{clear, color, style, cursor};
use termion::cursor::DetectCursorPos;
use termion::event::{Key};
use termion::input::TermRead;
use regex::Regex;
use rusqlite::{Connection, params_from_iter};

use buddy_utils::format_name;
use task::{Task, format_datetime};

const TASK_FIELDS: [&str; 9] = [
    "id",
    "title",
    "description",
    "due_date",
    "priority",
    "status",
    "created_at",
    "updated_at",
    "completed_at"
];

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
            "{}{}{}{}How can I help you {}?{}\n\r",
            clear::All,
            cursor::Goto(1,1),
            cursor::Hide,
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
            "{}{}{}Would you like me to help with another task? {}y/n{}\n\r{}{}",
            clear::All,
            cursor::Goto(1,1),
            color::Fg(color::Cyan),
            color::Fg(color::Yellow),
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
        "What is the DUE DATE associated with this task?",
        "On a scale of 1-5, what is the PRIORITY of this task?",
        "Can you give a current STATUS?",
    ];

    let mut idx = 0;
    while idx < prompt_items.len(){
        let field = prompt_items[idx];
        // Due Date precheck
        if idx == 2{
            write!(
                stdout(),
                "{}Is there a DUE DATE associated with this task? {}y/n\n\r {}> {}",
                color::Fg(color::Cyan),
                color::Fg(color::Yellow),
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
                "{}Have you started working on this task yet? {}y/n \n\r {}> {}",
                color::Fg(color::Cyan),
                color::Fg(color::Yellow),
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
            "{}{}\n\r{}{}{}> {}",
            color::Fg(color::Cyan),
            field,
            color::Fg(color::Yellow),
            if idx == 2 || idx == 4 { 
                if idx == 2 {
                    "  Please format in mm-dd-yyyy\n\r"
                } else {
                    "  Please give a estimated completion percentage (1-100)\n\r"
                }
            } else {""},
            color::Fg(color::Red),
            color::Fg(color::Reset),
        ).unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        for key in stdin().keys(){
            match key.unwrap(){
                Key::Esc => {
                    return;
                },
                Key::Left => {
                    if stdout().cursor_pos().unwrap().0 > 1{
                        write!(stdout(), "{}", cursor::Left(1)).unwrap();
                    }
                },
                Key::Right => {
                    if stdout().cursor_pos().unwrap().0 < input.len() as u16 {
                        write!(stdout(), "{}", cursor::Right(1)).unwrap();
                    }
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
                                input.clear();
                                write!(
                                    stdout(),
                                    "\r{}{}{}Invalid Due Date value{}",
                                    cursor::Hide,
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                ).unwrap();
                                stdout().flush().unwrap();
                                sleep(Duration::from_millis(1500));
                                // Redo current prompt iteration
                                write!(
                                    stdout(),
                                    "\r{}{}> {}{}{}",
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                    cursor::Show,
                                    cursor::BlinkingUnderline
                                ).unwrap();
                                stdout().flush().unwrap();
                                continue;
                            }
                        },
                        3 => {
                            if let Ok(valid) = input.parse::<u32>(){
                                match valid {
                                    1..=5 => {
                                        priority = valid
                                    },
                                    _ => {
                                        input.clear();
                                        write!(
                                            stdout(),
                                            "\r{}{}{}Invalid priority value{}",
                                            cursor::Hide,
                                            clear::CurrentLine,
                                            color::Fg(color::Red),
                                            color::Fg(color::Reset),
                                        ).unwrap();
                                        stdout().flush().unwrap();
                                        sleep(Duration::from_millis(1500));
                                        // Redo current prompt iteration
                                        write!(
                                            stdout(),
                                            "\r{}{}> {}{}{}",
                                            clear::CurrentLine,
                                            color::Fg(color::Red),
                                            color::Fg(color::Reset),
                                            cursor::Show,
                                            cursor::BlinkingUnderline
                                        ).unwrap();
                                        stdout().flush().unwrap();
                                        continue;
                                    }
                                }
                            } else{
                                input.clear();
                                write!(
                                    stdout(),
                                    "\r{}{}{}Invalid priority value{}",
                                    cursor::Hide,
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                ).unwrap();
                                stdout().flush().unwrap();
                                sleep(Duration::from_millis(1500));
                                // Redo current prompt iteration
                                write!(
                                    stdout(),
                                    "\r{}{}> {}{}{}",
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                    cursor::Show,
                                    cursor::BlinkingUnderline
                                ).unwrap();
                                stdout().flush().unwrap();
                                continue;
                            }
                        },
                        _ => {
                            if let Ok(valid) = input.parse::<u32>(){
                                match valid{
                                    0 => {},
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
                                        input.clear();
                                        write!(
                                            stdout(),
                                            "\r{}{}{}Invalid completion percentage value{}",
                                            cursor::Hide,
                                            clear::CurrentLine,
                                            color::Fg(color::Red),
                                            color::Fg(color::Reset),
                                        ).unwrap();
                                        stdout().flush().unwrap();
                                        sleep(Duration::from_millis(1500));
                                        // Redo current prompt iteration
                                        write!(
                                            stdout(),
                                            "\r{}{}> {}{}{}",
                                            clear::CurrentLine,
                                            color::Fg(color::Red),
                                            color::Fg(color::Reset),
                                            cursor::Show,
                                            cursor::BlinkingUnderline
                                        ).unwrap();
                                        stdout().flush().unwrap();
                                        continue;
                                    }
                                }
                            } else {
                                input.clear();
                                write!(
                                    stdout(),
                                    "\r{}{}{}Invalid completion percentage value{}",
                                    cursor::Hide,
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                ).unwrap();
                                stdout().flush().unwrap();
                                sleep(Duration::from_millis(1500));
                                // Redo current prompt iteration
                                write!(
                                    stdout(),
                                    "\r{}{}> {}{}{}",
                                    clear::CurrentLine,
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                    cursor::Show,
                                    cursor::BlinkingUnderline
                                ).unwrap();
                                stdout().flush().unwrap();
                                continue;
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
        idx += 1;
    }

    let mut task = Task::new(title, description, due_date, priority, status);
    if task.status == String::from("Completed"){
        task.completed_at = Some(Local::now());
    }


    let mut query_field = String::from("INSERT INTO tasks (title, description, priority, status, created_at, updated_at");
    let mut param_fields = String::from(" VALUES (?, ?, ?, ?, ?, ?");
    let mut parameters: Vec<String> = Vec::from([task.title, task.description, task.priority.to_string(), task.status, task.created_at.to_rfc3339(), task.updated_at.to_rfc3339()]);

    if let Some(_) = task.due_date{
        query_field.push_str(", due_date");
        param_fields.push_str(", ?");
        parameters.push(task.due_date.unwrap().to_rfc3339());
    }
    if let Some(_) = task.completed_at{
        query_field.push_str(", completed_at");
        param_fields.push_str(", ?");
        parameters.push(task.completed_at.unwrap().to_rfc3339());
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
            "{}{}{}Here is what I have found: \n\r",
            clear::All,
            cursor::Goto(1,1),
            color::Fg(color::Green),
        ).unwrap();

        if let Some(entry) = row.next().unwrap(){
            let id: u64 = entry.get(0).unwrap();
            let title: String = entry.get(1).unwrap();
            let description: String = entry.get(2).unwrap();
            let due_date: Option<DateTime<Local>> = if entry.get::<usize, Option<String>>(3).unwrap().is_none(){
                None
            } else{
                Some(convert_datetime(entry.get::<usize, Option<String>>(3).unwrap().unwrap()).unwrap())
            };

            let priority: u32 = entry.get(4).unwrap();
            let status: String = entry.get(5).unwrap();
            let created_at: DateTime<Local> = convert_datetime(entry.get(6).unwrap()).unwrap();
            let updated_at: DateTime<Local> = convert_datetime(entry.get(7).unwrap()).unwrap();
            let completed_at: Option<DateTime<Local>> = if entry.get::<usize, Option<String>>(8).unwrap().is_none(){
                None
            } else{
                Some(convert_datetime(entry.get::<usize, Option<String>>(8).unwrap().unwrap()).unwrap())
            };
            let task = Task::from_db(
                id,
                title,
                description,
                due_date,
                priority,
                status,
                created_at,
                updated_at,
                completed_at,
            );

            for (idx, field) in task.into_iter().enumerate(){
                write!(
                    stdout(),
                    "{}{}  {}: {}{}{}{}",
                    cursor::Goto(1, idx as u16 + 2),
                    color::Fg(color::Cyan),
                    TASK_FIELDS[idx],
                    cursor::Goto(17, idx as u16 + 2),
                    color::Fg(color::Green),
                    field,
                    color::Fg(color::Reset)
                ).unwrap();
                stdout().flush().unwrap();
            }
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
                color::Fg(color::Reset),
            ).unwrap();
        }

        write!(
            stdout(),
            "\n\r{}Please hit enter to continue.{}",
            color::Fg(color::Green),
            color::Fg(color::Reset)
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
        let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?").unwrap();
        let mut row = stmt.query(&[&id]).unwrap();
        if let Some(entry) = row.next().unwrap(){
            let id: u64 = entry.get(0).unwrap();
            let title: String = entry.get(1).unwrap();
            let description: String = entry.get(2).unwrap();
            let due_date: Option<DateTime<Local>> = if entry.get::<usize, Option<String>>(3).unwrap().is_none(){
                None
            } else{
                Some(convert_datetime(entry.get::<usize, Option<String>>(3).unwrap().unwrap()).unwrap())
            };

            let priority: u32 = entry.get(4).unwrap();
            let status: String = entry.get(5).unwrap();
            let created_at: DateTime<Local> = convert_datetime(entry.get(6).unwrap()).unwrap();
            let updated_at: DateTime<Local> = convert_datetime(entry.get(7).unwrap()).unwrap();
            let completed_at: Option<DateTime<Local>> = if entry.get::<usize, Option<String>>(8).unwrap().is_none(){
                None
            } else{
                Some(convert_datetime(entry.get::<usize, Option<String>>(8).unwrap().unwrap()).unwrap())
            };
            let mut task = Task::from_db(
                id,
                title,
                description,
                due_date,
                priority,
                status,
                created_at,
                updated_at,
                completed_at,
            );

            write!(
                stdout(),
                "{}{}{}{}",
                clear::All,
                cursor::Goto(1,1),
                cursor::Show,
                cursor::BlinkingUnderline
            ).unwrap();

            let mut current = 1;
            for (idx, field) in task.clone().into_iter().enumerate(){
                if idx == current {
                    write!(
                        stdout(),
                        "{}{}{}: {}{}> {}{}{}",
                        cursor::Goto(1, idx as u16 + 1),
                        color::Fg(color::Cyan),
                        TASK_FIELDS[idx],
                        cursor::Goto(15, idx as u16 + 1),
                        color::Fg(color::Red),
                        color::Fg(color::Yellow),
                        field,
                        color::Fg(color::Reset)
                    ).unwrap();
                } else {
                    write!(
                        stdout(),
                        "{}{}{}: {}{}{}",
                        cursor::Goto(1, idx as u16 + 1),
                        color::Fg(color::Cyan),
                        TASK_FIELDS[idx],
                        color::Fg(color::Reset),
                        cursor::Goto(15, idx as u16 + 1),
                        field,
                    ).unwrap();
                }
                stdout().flush().unwrap();
            }
            let mut task_items: Vec<String> = task.clone().into_iter().collect();
            write!(
                stdout(),
                "\n\n\r{}Use tab to submit an entry change\n\rEsc to quit / enter to proceed{}",
                color::Fg(color::Yellow),
                cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1)
            ).unwrap();
            stdout().flush().unwrap();

            let mut input = task_items[current].clone();
            for key in stdin().keys(){
                match key.unwrap(){
                    Key::Esc => {
                        return;
                    },
                    Key::Ctrl('u') | Key::Ctrl('\x7f') => {
                        let spot = stdout().cursor_pos().unwrap();
                        input.clear();
                        write!(
                            stdout(),
                            "{}{}{}{}: {}{}> {}{}{}",
                            cursor::Goto(1, spot.1),
                            clear::CurrentLine,
                            color::Fg(color::Cyan),
                            TASK_FIELDS[current],
                            cursor::Goto(15, spot.1),
                            color::Fg(color::Red),
                            color::Fg(color::Yellow),
                            input,
                            color::Fg(color::Reset),
                        ).unwrap();
                    },
                    Key::Delete | Key::Backspace => {
                        let spot = stdout().cursor_pos().unwrap();
                        if spot.0 > 17 && spot.0 <= 17 + input.len() as u16 {
                            let index = spot.0 - 17 - 1;
                            input.remove(index as usize);
                            write!(
                                stdout(),
                                "{}{}{}{}: {}{}> {}{}{}{}",
                                cursor::Goto(1, spot.1),
                                clear::CurrentLine,
                                color::Fg(color::Cyan),
                                TASK_FIELDS[current],
                                cursor::Goto(15, spot.1),
                                color::Fg(color::Red),
                                color::Fg(color::Yellow),
                                input,
                                color::Fg(color::Reset),
                                cursor::Goto(spot.0 - 1, spot.1),
                            ).unwrap();
                        }
                    },
                    Key::Up => {
                        write!(
                            stdout(),
                            "{}{}{}{}: {}{}{}",
                            cursor::Goto(1, current as u16 + 1),
                            clear::CurrentLine,
                            color::Fg(color::Cyan),
                            TASK_FIELDS[current],
                            cursor::Goto(15, current as u16 + 1),
                            color::Fg(color::Reset),
                            task_items[current],
                        ).unwrap();
                        match current {
                            2 | 3 | 4 | 5 => {
                                current -= 1;
                            },
                            8 => {
                                current = 5;
                            },
                            _ => {}
                        }
                        if current == 3 || current == 8{
                            write!(
                                stdout(),
                                "{}{}{}Please provide the date in mm-dd-yyyy format or None{}",
                                cursor::Goto(1, current as u16 + 1),
                                clear::CurrentLine,
                                color::Fg(color::Yellow),
                                color::Fg(color::Reset)
                            ).unwrap();
                            stdout().flush().unwrap();
                            sleep(Duration::from_millis(1800));
                        } else if current == 4 {
                            write!(
                                stdout(),
                                "{}{}{}Please provide the priority as a number between 1 and 5{}",
                                cursor::Goto(1, current as u16 + 1),
                                clear::CurrentLine,
                                color::Fg(color::Yellow),
                                color::Fg(color::Reset)
                            ).unwrap();
                            stdout().flush().unwrap();
                            sleep(Duration::from_millis(1800));
                        } else if current == 5 {
                            write!(
                                stdout(),
                                "{}{}{}Please provide the priority as a number between 1 and 100{}",
                                cursor::Goto(1, current as u16 + 1),
                                clear::CurrentLine,
                                color::Fg(color::Yellow),
                                color::Fg(color::Reset)
                            ).unwrap();
                            stdout().flush().unwrap();
                            sleep(Duration::from_millis(1800));
                        }
                        write!(
                            stdout(),
                            "{}{}{}{}: {}{}> {}{}{}",
                            cursor::Goto(1, current as u16 + 1),
                            clear::CurrentLine,
                            color::Fg(color::Cyan),
                            TASK_FIELDS[current],
                            cursor::Goto(15, current as u16 + 1),
                            color::Fg(color::Red),
                            color::Fg(color::Yellow),
                            task_items[current],
                            cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                        ).unwrap();
                        input = task_items[current].clone();
                    },
                    Key::Down => {
                        write!(
                            stdout(),
                            "{}{}{}{}: {}{}{}",
                            cursor::Goto(1, current as u16 + 1),
                            clear::CurrentLine,
                            color::Fg(color::Cyan),
                            TASK_FIELDS[current],
                            cursor::Goto(15, current as u16 + 1),
                            color::Fg(color::Reset),
                            task_items[current],
                        ).unwrap();
                        match current {
                            1 | 2 | 3| 4 => {
                                current += 1;
                            },
                            5 => {
                                current = 8;
                            },
                            _ => {}
                        }
                        if current == 3 || current == 8{
                            write!(
                                stdout(),
                                "{}{}{}Please provide the date in mm-dd-yyyy format or None{}",
                                cursor::Goto(1, current as u16 + 1),
                                clear::CurrentLine,
                                color::Fg(color::Yellow),
                                color::Fg(color::Reset)
                            ).unwrap();
                            stdout().flush().unwrap();
                            sleep(Duration::from_millis(1800));
                        } else if current == 4 {
                            write!(
                                stdout(),
                                "{}{}{}Please provide the priority as a number between 1 and 5{}",
                                cursor::Goto(1, current as u16 + 1),
                                clear::CurrentLine,
                                color::Fg(color::Yellow),
                                color::Fg(color::Reset)
                            ).unwrap();
                            stdout().flush().unwrap();
                            sleep(Duration::from_millis(1800));
                        } else if current == 5 {
                            write!(
                                stdout(),
                                "{}{}{}Please provide the priority as a number between 1 and 100{}",
                                cursor::Goto(1, current as u16 + 1),
                                clear::CurrentLine,
                                color::Fg(color::Yellow),
                                color::Fg(color::Reset)
                            ).unwrap();
                            stdout().flush().unwrap();
                            sleep(Duration::from_millis(1800));
                        }
                        write!(
                            stdout(),
                            "{}{}{}{}: {}{}> {}{}{}",
                            cursor::Goto(1, current as u16 + 1),
                            clear::CurrentLine,
                            color::Fg(color::Cyan),
                            TASK_FIELDS[current],
                            cursor::Goto(15, current as u16 + 1),
                            color::Fg(color::Red),
                            color::Fg(color::Yellow),
                            task_items[current],
                            cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                        ).unwrap();
                        input = task_items[current].clone();
                    },
                    Key::Left => {
                        if stdout().cursor_pos().unwrap().0 > 17 {
                            write!(stdout(), "{}", cursor::Left(1)).unwrap();
                        }
                    },
                    Key::Right => {
                        if stdout().cursor_pos().unwrap().0 < 17 + input.len() as u16 {
                            write!(stdout(), "{}", cursor::Right(1)).unwrap();
                        }
                    },

                    Key::Char('\t') => {
                        let old_input = input.clone(); 
                        match current {
                            1 => { task.title = input.to_string(); },
                            2 => { task.description = input.to_string(); },
                            3 => { 
                                if let Ok(valid) = validate_date_input(&input){
                                    let date = NaiveDate::parse_from_str(&valid, "%m-%d-%Y").unwrap();
                                    let time = NaiveTime::from_hms_opt(0,0,0).unwrap();
                                    let datetime = NaiveDateTime::new(date, time);
                                    task.due_date = Some(Local.from_local_datetime(&datetime).unwrap());
                                    input = format_datetime(task.due_date.unwrap());
                                } else if input == String::from("None") {
                                    task.completed_at = None;
                                } else {
                                    input = task_items[current].clone();
                                    write!(
                                        stdout(),
                                        "\r{}{}{}{}Invalid Due Date value: {}Must be a valid date in mm-dd-yyyy format or None{}",
                                        cursor::Hide,
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        color::Fg(color::Reset),
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    sleep(Duration::from_millis(2500));
                                    // Redo current prompt iteration
                                    write!(
                                        stdout(),
                                        "{}{}{}{}: {}{}> {}{}{}{}{}",
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Cyan),
                                        TASK_FIELDS[current],
                                        cursor::Goto(15, current as u16 + 1),
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        task_items[current],
                                        cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                                        cursor::Show,
                                        cursor::BlinkingUnderline
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    continue;
                                }
                            },
                            4 => { 
                                if let Ok(valid) = input.parse::<u32>(){
                                    match valid {
                                        1..=5 => {
                                            task.priority = valid; 
                                        },
                                        _ => {
                                            input = task_items[current].clone();
                                            write!(
                                                stdout(),
                                                "\r{}{}{}{}Invalid Priority value: {}Must be a number between 1 and 5{}",
                                                cursor::Hide,
                                                cursor::Goto(1, current as u16 + 1),
                                                clear::CurrentLine,
                                                color::Fg(color::Red),
                                                color::Fg(color::Yellow),
                                                color::Fg(color::Reset),
                                            ).unwrap();
                                            stdout().flush().unwrap();
                                            sleep(Duration::from_millis(2500));
                                            // Redo current prompt iteration
                                            write!(
                                                stdout(),
                                                "{}{}{}{}: {}{}> {}{}{}{}{}",
                                                cursor::Goto(1, current as u16 + 1),
                                                clear::CurrentLine,
                                                color::Fg(color::Cyan),
                                                TASK_FIELDS[current],
                                                cursor::Goto(15, current as u16 + 1),
                                                color::Fg(color::Red),
                                                color::Fg(color::Yellow),
                                                task_items[current],
                                                cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                                                cursor::Show,
                                                cursor::BlinkingUnderline
                                            ).unwrap();
                                            stdout().flush().unwrap();
                                            continue;
                                        }
                                    }
                                } else {
                                    input = task_items[current].clone();
                                    write!(
                                        stdout(),
                                        "\r{}{}{}{}Invalid Priority value: {}Must be a number between 1 and 5{}",
                                        cursor::Hide,
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        color::Fg(color::Reset),
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    sleep(Duration::from_millis(2500));
                                    // Redo current prompt iteration
                                    write!(
                                        stdout(),
                                        "{}{}{}{}: {}{}> {}{}{}{}{}",
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Cyan),
                                        TASK_FIELDS[current],
                                        cursor::Goto(15, current as u16 + 1),
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        task_items[current],
                                        cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                                        cursor::Show,
                                        cursor::BlinkingUnderline
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    continue;
                                }
                            },
                            5 => { 
                                if let Ok(valid) = input.parse::<u32>(){
                                    match valid {
                                        1..=35 => {
                                            task.status = String::from("Starting");
                                            input = String::from("Starting");
                                            task.completed_at = None;
                                        },
                                        36..=70 => {
                                            task.status = String::from("Working");
                                            input = String::from("Working");
                                            task.completed_at = None;
                                        },
                                        71..=99 => {
                                            task.status = String::from("Finishing");
                                            input = String::from("Finishing");
                                            task.completed_at = None;
                                        },
                                        100 => {
                                            task.status = String::from("Completed");
                                            input = String::from("Completed");
                                            task.completed_at = Some(Local::now());
                                        }
                                        _ => {
                                            input = task_items[current].clone();
                                            write!(
                                                stdout(),
                                                "\r{}{}{}{}Invalid Status value: {}Must be a percentage between 1 and 100{}",
                                                cursor::Hide,
                                                cursor::Goto(1, current as u16 + 1),
                                                clear::CurrentLine,
                                                color::Fg(color::Red),
                                                color::Fg(color::Yellow),
                                                color::Fg(color::Reset),
                                            ).unwrap();
                                            stdout().flush().unwrap();
                                            sleep(Duration::from_millis(2500));
                                            // Redo current prompt iteration
                                            write!(
                                                stdout(),
                                                "{}{}{}{}: {}{}> {}{}{}{}{}",
                                                cursor::Goto(1, current as u16 + 1),
                                                clear::CurrentLine,
                                                color::Fg(color::Cyan),
                                                TASK_FIELDS[current],
                                                cursor::Goto(15, current as u16 + 1),
                                                color::Fg(color::Red),
                                                color::Fg(color::Yellow),
                                                task_items[current],
                                                cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                                                cursor::Show,
                                                cursor::BlinkingUnderline
                                            ).unwrap();
                                            stdout().flush().unwrap();
                                            continue;
                                        }
                                    }
                                    write!(
                                        stdout(),
                                        "{}{}{}{}: {}{}{}{}",
                                        cursor::Goto(1, task_items.len() as u16),
                                        clear::CurrentLine,
                                        color::Fg(color::Cyan),
                                        TASK_FIELDS[8],
                                        cursor::Goto(15, task_items.len() as u16),
                                        color::Fg(color::Reset),
                                        if let Some(comp) = task.completed_at {
                                            format_datetime(comp)
                                        } else { "None".to_string() },
                                        cursor::Goto(17 + old_input.len() as u16, current as u16 + 1)
                                    ).unwrap();
                                } else {
                                    input = task_items[current].clone();
                                    write!(
                                        stdout(),
                                        "\r{}{}{}{}Invalid Status value: {}Must be a percentage between 1 and 100{}",
                                        cursor::Hide,
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        color::Fg(color::Reset),
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    sleep(Duration::from_millis(2500));
                                    // Redo current prompt iteration
                                    write!(
                                        stdout(),
                                        "{}{}{}{}: {}{}> {}{}{}{}{}",
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Cyan),
                                        TASK_FIELDS[current],
                                        cursor::Goto(15, current as u16 + 1),
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        task_items[current],
                                        cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                                        cursor::Show,
                                        cursor::BlinkingUnderline
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    continue;
                                } 
                            },
                            8 => { 
                                if let Ok(valid) = validate_date_input(&input){
                                    let date = NaiveDate::parse_from_str(&valid, "%m-%d-%Y").unwrap();
                                    let time = NaiveTime::from_hms_opt(0,0,0).unwrap();
                                    let datetime = NaiveDateTime::new(date, time);
                                    task.completed_at = Some(Local.from_local_datetime(&datetime).unwrap());
                                    input = format_datetime(task.due_date.unwrap());
                                } else if input == String::from("None") {
                                    task.completed_at = None;
                                } else {
                                    input = task_items[current].clone();
                                    write!(
                                        stdout(),
                                        "\r{}{}{}{}Invalid Completion Date value: {}Must be a valid date in mm-dd-yyyy format or None{}",
                                        cursor::Hide,
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        color::Fg(color::Reset),
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    sleep(Duration::from_millis(2500));
                                    // Redo current prompt iteration
                                    write!(
                                        stdout(),
                                        "{}{}{}{}: {}{}> {}{}{}{}{}",
                                        cursor::Goto(1, current as u16 + 1),
                                        clear::CurrentLine,
                                        color::Fg(color::Cyan),
                                        TASK_FIELDS[current],
                                        cursor::Goto(15, current as u16 + 1),
                                        color::Fg(color::Red),
                                        color::Fg(color::Yellow),
                                        task_items[current],
                                        cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                                        cursor::Show,
                                        cursor::BlinkingUnderline
                                    ).unwrap();
                                    stdout().flush().unwrap();
                                    continue;
                                }
                            },
                            _ => {}
                        }
                        write!(
                            stdout(),
                            "{}{}...Updating{}",
                            cursor::Goto(17 + old_input.len() as u16, current as u16 + 1),
                            color::Fg(color::Green),
                            color::Fg(color::Reset),
                        ).unwrap();
                        stdout().flush().unwrap();
                        task_items[current] = input;
                        input = task_items[current].clone();
                        sleep(Duration::from_millis(500));
                        write!(
                            stdout(),
                            "{}{}{}{}: {}{}> {}{}{}{}{}",
                            cursor::Goto(1, current as u16 + 1),
                            clear::CurrentLine,
                            color::Fg(color::Cyan),
                            TASK_FIELDS[current],
                            cursor::Goto(15, current as u16 + 1),
                            color::Fg(color::Red),
                            color::Fg(color::Yellow),
                            task_items[current],
                            cursor::Goto(17 + task_items[current].len() as u16, current as u16 + 1),
                            cursor::Show,
                            cursor::BlinkingUnderline
                        ).unwrap();
                    },
                    Key::Char('\n') => {
                        task.updated_at = Local::now();
                        break;
                    },
                    Key::Char(c) => {
                        let spot = stdout().cursor_pos().unwrap();
                        if spot.0 >= 17 && spot.0 <= 17 + input.len() as u16{
                            let index = spot.0 - 17;
                            input.insert(index as usize, c);
                            write!(
                                stdout(),
                                "{}{}{}{}: {}{}> {}{}{}{}",
                                cursor::Goto(1, spot.1),
                                clear::CurrentLine,
                                color::Fg(color::Cyan),
                                TASK_FIELDS[current],
                                cursor::Goto(15, spot.1),
                                color::Fg(color::Red),
                                color::Fg(color::Yellow),
                                input,
                                color::Fg(color::Reset),
                                cursor::Goto(spot.0 + 1, spot.1)
                            ).unwrap();
                        }
                    }
                    _ => {}
                }
                stdout().flush().unwrap();
            }
            let mut query = String::from("UPDATE tasks SET title = ?, description = ?, priority = ?, status = ?, updated_at = ?");
            let mut parameters: Vec<String> = Vec::from([task.title, task.description, task.priority.to_string(), task.status, task.updated_at.to_rfc3339()]);

            if let Some(_) = task.due_date{
                query.push_str(", due_date = ?");
                parameters.push(task.due_date.unwrap().to_rfc3339());
            }
            if let Some(_) = task.completed_at{
                query.push_str(", completed_at = ?");
                parameters.push(task.completed_at.unwrap().to_rfc3339());
            }
            query.push_str("WHERE id = ?;");
            parameters.push(task.id.unwrap().to_string());
            if let Err(err) = conn.execute(
                &query, params_from_iter(parameters.iter())){
                write!(
                    stdout(),
                    "{}{}{}There has been an error with updating your task. Error: {}{}",
                    clear::All,
                    cursor::Goto(1,1),
                    color::Fg(color::Red),
                    err,
                    color::Fg(color::Reset),
                ).unwrap();
            } else{
                write!(
                    stdout(),
                    "{}{}{}Entry has been updated successfully!{}",
                    clear::All,
                    cursor::Goto(1,1),
                    color::Fg(color::Green),
                    color::Fg(color::Reset),
                ).unwrap();
            }
            stdout().flush().unwrap();
            sleep(Duration::from_millis(2500));
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
    } else { return; }
}

fn delete_task(conn: &Connection) {
    if let Ok(id) = display_all_tasks(conn, "delete"){
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

    if items.len() == 0 {
        write!(
            stdout(),
            "{}{}{}{}There are not task entries to {}.{}\n\r",
            cursor::Hide,
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Red),
            action,
            color::Fg(color::Reset)
        ).unwrap();
        stdout().flush().unwrap();
        sleep(Duration::from_millis(2000));
        return Err(());
    }

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
                "{}{}    {}) {}",
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
                        cursor::Goto(1, selected as u16 + 2),
                        clear::CurrentLine,
                        color::Fg(color::Red),
                        &items[selected as usize].0,
                        &items[selected as usize].1.to_uppercase(),
                        color::Fg(color::Reset),
                    ).unwrap();
                    stdout().flush().unwrap();
                }
            },
            Key::Down => {
                if (selected as usize) < items.len() - 1 {
                    write!(
                        stdout(),
                        "{}{}{}    {}) {}",
                        cursor::Goto(1, selected as u16 + 2),
                        clear::CurrentLine,
                        color::Fg(color::Reset),
                        &items[selected as usize].0,
                        &items[selected as usize].1.to_uppercase(),
                    ).unwrap();
                    selected += 1;
                    write!(
                        stdout(),
                        "{}{}{}  > {}) {}{}",
                        cursor::Goto(1, selected as u16 + 2),
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
                write!(
                    stdout(), 
                    "{}{}",
                    clear::All,
                    cursor::Goto(1,1),
                ).unwrap();
                return Ok(items[selected as usize].0 as u32);
            },
            _ => {}
        }
    }
    Err(())
}

fn validate_date_input(date: &String) -> Result<String, ()>{
    let date_regex = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap();
    if date_regex.is_match(date){
        Ok(date.to_string())
    } else {
        Err(())
    }
}

fn convert_datetime(value: String) -> Result<DateTime<Local>, ()>{
    if let Ok(parsed_date) = DateTime::parse_from_rfc3339(&value){
        stdout().flush().unwrap();
        let datetime = parsed_date.with_timezone(&Local);
        Ok(datetime)
    } else {
        stdout().flush().unwrap();
        Err(())
    }
}