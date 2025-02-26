use anyhow::{Context, Result, anyhow};
use std::{fs, path::Path, process::Command};

/// Sets up a Injection Forge sources from Git repository by either cloning it (if it doesn't exist) or updating it (if it already exists).
///
/// # Arguments
///
/// * `repo_url` - The Injection Forge sources Git repository url
/// * `build_dir` - The build directory for Injection Forge
///
/// # Returns
///
/// * `Ok(())` if the the Injection Forge sources was setup successfully .
/// * `Err` if an error occurs during any of the operations.
///`
pub fn setup(repo_url: &Path, build_dir: &Path) -> Result<()> {
    println!("Running setup");
    let repo_url = repo_url.to_string_lossy().to_string();
    let build_dir = build_dir.to_string_lossy().to_string();

    if !is_git_repo(&build_dir) {
        if Path::new(&build_dir).is_dir() {
            fs::remove_dir_all(build_dir.clone())?; // Remove the existing directory
        }
        // TODO Add option to select banch, commit, tags ...
        println!("Clone {} repo to {}", &repo_url, &build_dir);
        clone_repo_without_history(&repo_url, &build_dir)?; // Clone the repository without history
    } else {
        println!("Update repo in {}", &build_dir);
        update_repo_without_history(&build_dir)?;
    }
    Ok(())
}

/// Clones a Git repository to a specified directory without the commit history.
///
/// # Arguments
///
/// * `repo_url` - A string slice that holds the URL of the repository to be cloned.
/// * `target_dir` - A string slice that holds the path to the directory where the repository
///   should be cloned.
///
/// # Returns
///
/// This function returns a `Result<()>`. On success, it returns `Ok(())`, indicating that the repository
/// was cloned successfully without history. On failure, it returns an error wrapped in `anyhow::Error`.
///
/// # Example
///
/// ```rust
/// clone_repo_without_history("https://github.com/rust-lang/rust.git", "rust_repo")?;
/// ```
fn clone_repo_without_history(repo_url: &str, target_dir: &str) -> Result<()> {
    // Execute the `git clone --depth 1` command
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1") // This ensures we only clone the latest snapshot, without history
        .arg(repo_url)
        .arg(target_dir)
        .status()
        .context("Failed to execute git clone")?;

    // Check if the git clone command was successful
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to clone repository").into())
    }
}

/// Checks if the provided path is a valid Git repository.
///
/// # Arguments
///
/// * `path` - The path to the directory you want to check.
///
/// # Returns
///
/// * `Ok(true)` if the directory is a Git repository.
/// * `Ok(false)` if the directory is not a Git repository.
/// * `Err(String)` if there was an error while executing the command.
fn is_git_repo(path: &str) -> bool {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output();

    match output {
        Ok(output) if output.status.success() => true, // Directory is a Git repository
        _ => false,                                    // Not a Git repository
    }
}

/// Checks if the given path is a valid Git repository and if it's up-to-date with its remote.
///
/// # Arguments
///
/// * `path` - The path to the directory that you want to check.
///
/// # Returns
///
/// * `Ok(true)` if the repository is up-to-date.
/// * `Ok(false)` if the repository is behind the remote or not up-to-date.
/// * `Err(String)` if there is any error, such as the directory not being a Git repository or failure to fetch updates.
fn is_git_repo_latest(path: &str) -> Result<bool> {
    // Check if the directory is a git repository
    let is_git_repo = is_git_repo(path);

    if !is_git_repo {
        return Err(anyhow!("Not a git repository.".to_string())); // Return error if not a Git repository
    }

    // Fetch the latest changes from the remote repository
    let fetch_output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("fetch")
        .output();

    if fetch_output.is_err() {
        return Err(anyhow!(
            "Failed to fetch updates from the remote.".to_string()
        ));
    }

    // Check the repository's status to determine if it's up-to-date
    let status_output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .arg("-uno") // Only relevant info (avoiding additional output)
        .output()
        .map_err(|e| anyhow!(e.to_string()))?;

    let status_str = String::from_utf8_lossy(&status_output.stdout);

    // Check if the branch is up-to-date with the remote
    if status_str.contains("Your branch is up to date with") {
        Ok(true) // Repository is up-to-date
    } else if status_str.contains("Your branch is behind") {
        Ok(false) // Repository is not up-to-date
    } else {
        Err(anyhow!(
            "Unable to determine if repository is up-to-date.".to_string()
        )) // Unable to determine status
    }
}

/// Updates the repository to the latest commit from the remote without keeping the local history.
/// This will discard any local changes or commits and reset the repository to match the remote branch.
///
/// # Arguments
///
/// * `path` - The path to the directory that you want to update.
///
/// # Returns
///
/// * `Ok(())` if the update was successful.
/// * `Err(String)` if there was an error, such as the directory not being a Git repository or the update failing.
fn update_repo_without_history(path: &str) -> Result<()> {
    // Check if the directory is a git repository
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output();

    match output {
        Ok(output) if output.status.success() => {} // Directory is a Git repository
        _ => return Err(anyhow!("Not a git repository.".to_string())), // Return error if not a Git repository
    }

    // Fetch the latest changes from the remote repository
    let fetch_output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("fetch")
        .output();

    if fetch_output.is_err() {
        return Err(anyhow!(
            "Failed to fetch updates from the remote.".to_string()
        ));
    }

    // Reset the local repository to the latest commit from the remote
    let reset_output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("reset")
        .arg("--hard")
        .arg("origin/main") // Assuming the remote branch is `main`. // TODO To be improve
        .output();

    if reset_output.is_err() {
        return Err(anyhow!("Failed to reset the repository.".to_string()));
    }

    Ok(())
}
