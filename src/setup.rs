use anyhow::{Context, Result, anyhow};
use std::{fs, path::Path, process::Command};

/// Sets up an Injection Forge source from a Git repository by either cloning it (if it doesn't exist)
/// or updating it (if it already exists).
///
/// # Arguments
///
/// * `repo_url` - The Injection Forge sources Git repository URL.
/// * `repo_path` - The directory where the repository will be stored.
/// * `checkout_value` - The commit, branch, or tag to checkout.
///
/// # Returns
///
/// * `Ok(())` if the Injection Forge sources were set up successfully.
/// * `Err` if an error occurs during any of the operations.
pub fn setup(repo_url: &Path, repo_path: &Path, checkout_value: &str) -> Result<()> {
    println!("Running setup");

    let repo_url = repo_url
        .to_str()
        .ok_or(anyhow!("Repo URL is not a valid string"))?;
    let build_dir = repo_path
        .to_str()
        .ok_or(anyhow!("Build directory is not a valid string"))?;

    if !is_git_repo(build_dir) {
        if Path::new(build_dir).is_dir() {
            fs::remove_dir_all(build_dir)?; // Remove the existing directory
        }
        // TODO: Add option to select branch, commit, tags, etc.
        println!("Running git clone {} to {}", repo_url, build_dir);
        git_clone(repo_url, build_dir)?; // Clone the repository without history
    } else {
        println!("Running git fetch in {}", build_dir);
        git_fetch(build_dir)?;
    }

    println!("Running git checkout {}", checkout_value);
    git_checkout(build_dir, checkout_value)?;
    Ok(())
}

/// Clones a Git repository to a specified directory.
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
/// was cloned successfully. On failure, it returns an error wrapped in `anyhow::Error`.
///
/// # Example
///
/// ```rust
/// git_clone("https://github.com/rust-lang/rust.git", "rust_repo")?;
/// ```
fn git_clone(repo_url: &str, target_dir: &str) -> Result<()> {
    // Execute the `git clone --depth 1` command
    let status = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(target_dir)
        .status()
        .context("Failed to execute git clone")?;

    // Check if the git clone command was successful
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to clone repository").into())
    }
}

/// Checks out a specific commit, branch, or tag in the repository.
///
/// # Arguments
///
/// * `repo_path` - The path to the repository.
/// * `checkout_value` - The commit, branch, or tag to checkout.
///
/// # Returns
///
/// This function returns a `Result<()>`. On success, it returns `Ok(())`, indicating that the repository
/// was successfully checked out. On failure, it returns an error wrapped in `anyhow::Error`.
fn git_checkout(repo_path: &str, checkout_value: &str) -> Result<()> {
    // Execute the `git checkout <checkout_value>` command
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("checkout")
        .arg(checkout_value)
        .status()
        .context("Failed to execute git checkout")?;

    // Check if the git checkout command was successful
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to checkout the repository").into())
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
/// * `true` if the directory is a Git repository.
/// * `false` if the directory is not a Git repository.
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
fn git_fetch(path: &str) -> Result<()> {
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
    Ok(())
}
