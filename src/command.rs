use log::{debug, error};
use std::process::{Command, Stdio};

pub fn run_command(command: Vec<String>) {
    debug!("Running command: {:?}", command);
    match Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Err(e) => {
            error!("Error running command: {:?}", e);
        }
        Ok(child) => {
            debug!("Process spawned: {:?}, pid {}", command, child.id());
        }
    }
}
