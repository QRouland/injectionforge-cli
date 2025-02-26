use std::{fs, path::Path};

use anyhow::Result;

/// Cleans the build directory by removing all its contents.
///
/// # Arguments
///
/// * `build_dir` - The build directory path to clean.
///
/// # Returns
///
/// This function returns a `Result<()>`:
/// - `Ok(())` if the clean operation was successful.
/// - `Err(anyhow::Error)` if there was an error removing the build directory or its contents.
///
pub fn clean(build_dir: &Path) -> Result<()> {
    println!("Running clean");
    let build_dir = build_dir.to_string_lossy().to_string();
    if Path::new(&build_dir).is_dir() {
        fs::remove_dir_all(build_dir.clone())?;
    }
    Ok(())
}
