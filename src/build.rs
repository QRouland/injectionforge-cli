use anyhow::{Context, Ok, Result};
use std::{
    path::{self, Path, PathBuf},
    process::{Command, Stdio},
    str,
};

pub fn build_injectable_dll(build_dir: &Path, frida_code: &(&str, &str)) -> Result<Vec<PathBuf>> {
    println!("Running build injectable dll");
    let cargo = Command::new("cargo")
        .current_dir(build_dir.to_string_lossy().to_string())
        .env(frida_code.0, frida_code.1)
        .arg("build")
        .arg("--message-format=json-render-diagnostics")
        .arg("--lib")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to execute cargo to build injectable dll")?;

    let jq_query = "[.[] | select(.reason == \"compiler-artifact\") | select(.package_id | contains(\"injectionforge\")) | select(.target.kind[] == \"cdylib\")] | last | .filenames";

    let jq = Command::new("jq")
        .arg("-js")
        .arg(jq_query)
        .stdin(Stdio::from(cargo.stdout.unwrap()))
        .output()
        .context("Failed to execute jq to retrieve build injectable dll artifacts")?;

    if !jq.status.success() {
        return Err(anyhow::anyhow!("Failed to build injectable dll"));
    }
    let v: Vec<String> = serde_json::from_str(str::from_utf8(&jq.stdout)?)?;
    Ok(v.iter().map(|x| PathBuf::from(x)).collect())
}

pub fn build_proxy_dll(
    build_dir: &Path,
    dll_proxy: &Path,
    frida_code: &(&str, &str),
) -> Result<Vec<PathBuf>> {
    println!("Running build proxy dll");
    let dll_proxy = path::absolute(dll_proxy)?;
    let cargo = Command::new("cargo")
        .current_dir(build_dir.to_string_lossy().to_string())
        .env(frida_code.0, frida_code.1)
        .env("DLL_PROXY", dll_proxy.to_string_lossy().to_string())
        .arg("build")
        .arg("--lib")
        .arg("--message-format=json-render-diagnostics")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to execute cargo to build proxy dll")?;

    let jq_query = "[.[] | select(.reason == \"compiler-artifact\") | select(.package_id | contains(\"injectionforge\")) | select(.target.kind[] == \"cdylib\")] | last | .filenames";

    let jq = Command::new("jq")
        .arg("-js")
        .arg(jq_query)
        .stdin(Stdio::from(cargo.stdout.unwrap()))
        .output()
        .context("Failed to execute jq to retrieve build proxy dll artifacts")?;

    if !jq.status.success() {
        return Err(anyhow::anyhow!("Failed to build proxy dll"));
    }
    let v: Vec<String> = serde_json::from_str(str::from_utf8(&jq.stdout)?)?;
    Ok(v.iter().map(|x| PathBuf::from(x)).collect())
}

pub fn build_standalone_exe(build_dir: &Path, frida_code: &(&str, &str)) -> Result<Vec<PathBuf>> {
    println!("Running build standalone exe");
    let cargo = Command::new("cargo")
        .current_dir(build_dir.to_string_lossy().to_string())
        .env(frida_code.0, frida_code.1)
        .arg("build")
        .arg("--bin")
        .arg("standalone")
        .arg("--message-format=json-render-diagnostics")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to execute cargo to build standalone exe")?;

    let jq_query = "[.[] | select(.reason == \"compiler-artifact\") | select(.package_id | contains(\"injectionforge\")) | select(.target.kind[] == \"bin\")] | last | .filenames";

    let jq = Command::new("jq")
        .arg("-js")
        .arg(jq_query)
        .stdin(Stdio::from(cargo.stdout.unwrap()))
        .output()
        .context("Failed to execute jq to retrieve build standalone exe artifacts")?;

    if !jq.status.success() {
        return Err(anyhow::anyhow!("Failed to build standalone exe"));
    }
    let v: Vec<String> = serde_json::from_str(str::from_utf8(&jq.stdout)?)?;
    Ok(v.iter().map(|x| PathBuf::from(x)).collect())
}
