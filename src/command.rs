use std::process::Command;

/// Executes a command with the specified arguments and captures its output.
///
/// # Arguments
///
/// * `cmd` - The command to execute.
/// * `args` - The arguments to pass to the command.
///
/// # Returns
///
/// Returns a [`std::process::Output`] struct containing the output of the executed command,
/// including stdout, stderr, and the exit status.
///
/// # Panics
///
/// Panics with the message "Failed to execute command." if the command fails to start.
pub fn execute(command: &str) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    let cmd: &str = parts[0];
    let args: &[&str] = &parts[1..];

    let output: std::process::Output = Command::new(cmd).args(args).output().expect("Failed to execute command.");

    if output.status.success() {
        return String::from_utf8_lossy(&output.stdout).into_owned();
    } else {
        return String::from_utf8_lossy(&output.stderr).into_owned();
    }
}