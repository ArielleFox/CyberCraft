use std::process::{Command, Output};
use std::io::{self};

/// This function runs a terminal command and returns the result.
pub fn run_command(command: &str, args: &[&str]) -> io::Result<Output> {
    // Run the command with the given arguments
    let output = Command::new(command)
        .args(args)
        .output()?; // Executes the command and returns the result

    // Optionally, print the output to the console
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // Return the output of the command
    Ok(output)
}
