//! CLI utilities for the Billion Row Challenge.

use std::path::PathBuf;

use clap::Parser;

/// Returns the default output folder for generated data.
///
/// Considers the `data` folder as the default if run via cargo, the standalone binary uses the
/// current directory.
fn get_default_folder() -> PathBuf {
    let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") else {
        return PathBuf::from(".");
    };

    PathBuf::from(manifest_dir)
        .parent()
        .expect("Unknown cargo project structure")
        .join("data")
}

/// CLI for computing the solution to the Billion Row Challenge.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Path to output folder.
    #[arg(long, default_value_os_t = get_default_folder())]
    pub(crate) input_folder: PathBuf,

    /// Path to input file.
    #[arg(short, long)]
    pub(crate) input_file: String,
}

impl Cli {
    /// Returns the path to the input file.
    pub fn get_input_path(&self) -> PathBuf {
        self.input_folder.join(&self.input_file)
    }

    /// Parses the CLI arguments.
    pub(crate) fn create() -> Self {
        Self::parse()
    }
}
