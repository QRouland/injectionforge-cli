use std::str;

use crate::build::{build_injectable_dll, build_proxy_dll, build_standalone_exe};
use crate::clean::clean;
use crate::cli::Cli;
use crate::dist::dist;
use crate::setup::setup;
use crate::utils::check_command;
use anyhow::{Ok, Result, anyhow};
use clap::Parser;
use run::run_standalone_exe;

mod build;
mod clean;
mod cli;
mod dist;
mod run;
mod setup;
mod utils;

/// Checks if requirements are installed and available on the system.
///
/// This function calls verify that the requirements are installed
///
/// # Returns
///
/// This function returns a `Result<()>`:
/// - `Ok(())` if requierements are ok.
/// - `Err(anyhow::Error)` if a requirement is not available.
///
fn check_requirements() -> Result<()> {
    check_command("cargo", "--version")?; // Checks if cargo is installed
    check_command("git", "--version")?; // Checks if Git is installed
    check_command("jq", "--version")?; // Checks if jq is installed
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    println!(
        "injectionforge-cli - Version {}",
        VERSION.unwrap_or("unknown")
    );

    if !(args.clean
        || args.standalone_exe
        || args.run_standalone_exe.is_some()
        || args.injectable_dll
        || args.proxy_dll.is_some())
    {
        println!(
            "Nothing to do, use injectionforge-cli --help or injectionforge-cli --h to show help"
        );
        return Ok(());
    }

    let frida_code = if let Some(x) = &args.frida_code_file {
        ("FRIDA_CODE_FILE", x.to_str().unwrap())
    } else if let Some(x) = &args.frida_code_string {
        ("FRIDA_CODE", x.as_str())
    } else {
        return Err(anyhow!(
            "Either --frida-code-file or --frida-code-string should be provided"
        )); // should never be reached, has enforced with Clap
    };

    check_requirements()?;
    if args.clean {
        clean(&args.build_dir)?;
    }
    if args.standalone_exe || args.injectable_dll || args.proxy_dll.is_some() {
        setup(&args.repo_url, &args.build_dir, &args.repo_checkout)?;
        if args.standalone_exe {
            let artifacts = build_standalone_exe(&args.build_dir, &frida_code)?;
            dist(&artifacts, &args.dist_dir)?;
        }
        if args.injectable_dll {
            let artifacts = build_injectable_dll(&args.build_dir, &frida_code)?;
            dist(&artifacts, &args.dist_dir)?;
        }
        if args.proxy_dll.is_some() {
            let artifacts =
                build_proxy_dll(&args.build_dir, &args.proxy_dll.unwrap(), &frida_code)?;
            dist(&artifacts, &args.dist_dir)?;
        }
    }
    if args.run_standalone_exe.is_some() {
        run_standalone_exe(
            &args.build_dir,
            &frida_code,
            &args.run_standalone_exe.unwrap(),
        )?;
    }
    Ok(())
}
