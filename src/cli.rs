use clap::Parser;
use std::path::PathBuf;

/// injectionforge-cli is a command-line interface for injectionforge.
#[derive(Parser, Clone)]
#[command(
    version,
    about,
    long_about = None
)]
pub struct Cli {
    /// InjectionForge source repo url.
    #[arg(
        short,
        long,
        default_value = "https://github.com/dzervas/injectionforge"
    )]
    pub repo_url: PathBuf,
    /// Path of the build directory.
    #[arg(
        short,
        long,
        value_name("BUILD_DIR_PATH"),
        default_value = "./build/injectionforge"
    )]
    pub build_dir: PathBuf,

    /// Path of the dist directory.
    #[arg(short, long, value_name("DIST_DIR_PATH"), default_value = "./dist/")]
    pub dist_dir: PathBuf,

    /// Build standalone exe.
    #[arg(short, long, conflicts_with = "run_standalone_exe")]
    pub standalone_exe: bool,

    /// Run standalone exe with pid <PID>.
    #[arg(short= 'e', long , value_name("PID|PROCESS_NAME"), conflicts_with_all = ["injectable_dll", "proxy_dll", "standalone_exe"])]
    pub run_standalone_exe: Option<String>,

    /// Build injectable dll.
    #[arg(short, long, conflicts_with = "run_standalone_exe")]
    pub injectable_dll: bool,

    /// Build proxy dll for the dll at <PROXY_DLL_PATH>.
    #[arg(
        short,
        long,
        value_name("PROXY_DLL_PATH"),
        conflicts_with = "run_standalone_exe"
    )]
    pub proxy_dll: Option<PathBuf>,

    /// Frida code script as string to inject.
    #[arg(
        short = 'c',
        long,
        conflicts_with = "frida_code_file",
        required_unless_present("frida_code_file")
    )]
    pub frida_code_string: Option<String>,

    /// Frida code script as a file to inject.
    #[arg(
        short = 'f',
        long,
        value_name("FRIDA_CODE_FILE_PATH"),
        conflicts_with = "frida_code_string",
        required_unless_present("frida_code_string")
    )]
    pub frida_code_file: Option<PathBuf>,

    // TODO Add option de rename the generated dll, so, bin to a custom name
    // /// Name use for the output artifacts
    // #[arg(short, long)]
    // pub output_name: Option<String>,
    /// Clean the <BUILD_DIR_PATH> directory.
    #[arg(long)]
    pub clean: bool,
}
