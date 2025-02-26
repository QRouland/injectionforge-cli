use std::process::Command;

use anyhow::{Context, Result};

/// Checks if a command is installed and available on the system.
///
/// # Arguments
///
/// * `command` - The name of the command (e.g., "git", "jq") to check.
/// * `version_flag` - The flag used to retrieve the version (e.g., "--version").
///
/// # Returns
///
/// This function returns a `Result<()>`:
/// - `Ok(())` if the command is found and the version information is successfully retrieved.
/// - `Err(anyhow::Error)` if the command is not found or fails to execute.
///
/// # Example
///
/// ```rust
/// check_command("git", "--version")?; // Checks if Git is installed and prints its version
/// check_command("jq", "--version")?;  // Checks if jq is installed and prints its version
/// ```
pub fn check_command(command: &str, version_flag: &str) -> Result<()> {
    let command_status = Command::new(command)
        .arg(version_flag)
        .output()
        .context(format!("Failed to execute {} command", command))?;

    if command_status.status.success() {
        print!(
            "{} found : {}",
            command,
            String::from_utf8_lossy(&command_status.stdout)
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "{} is not available. Please install {} and made it available in your PATH.",
            command,
            command
        )
        .into())
    }
}
