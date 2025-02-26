use anyhow::{Context, Ok, Result};
use std::{
    path::Path,
    process::{Command, Stdio},
    str,
};

pub fn run_standalone_exe(build_dir: &Path, frida_code: &(&str, &str), pid: &str) -> Result<()> {
    println!("Running run standalone exe");
    let cargo = Command::new("cargo")
        .current_dir(build_dir.to_string_lossy().to_string())
        .env(frida_code.0, frida_code.1)
        .arg("run")
        .arg("--bin")
        .arg("standalone")
        .arg(pid)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .context("Failed to execute cargo to run standalone exe")?;

    if !cargo.status.success() {
        return Err(anyhow::anyhow!("Run standalone exe terminated with error"));
    }
    Ok(())
}
