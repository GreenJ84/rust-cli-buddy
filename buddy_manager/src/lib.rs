use std::process::{Command, Stdio};
use std::{thread::sleep, time::Duration, io::stdout};

use buddy_service::utils::{application, terminal};

pub fn spawn_program(program: &str) {
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
    if let Ok(mut spawn) = command.spawn() {
        let exit_status = spawn
            .wait()
            .expect(&format!("Failed to run {} program.", program));
        if !exit_status.success() {
            application::error_confirmation(&mut stdout(), "Sub Application exited with an error.");
        } else {
            println!(
                "\n\r{}",
                terminal::info_text(&format!("Exiting {}. Status: {}", program, exit_status))
            );
        }
    } else {
        application::error_confirmation(
            &mut stdout(),
            "Program spawning command failed to initialize.",
        );
    }

    sleep(Duration::from_millis(500));
}