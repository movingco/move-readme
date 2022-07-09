//! Generates a readme from a Move package.
//!
//! # Setup
//!
//! Install the CLI using Cargo:
//!
//! ```bash
//! cargo install move-readme
//!
//! # On Sui
//! cargo install move-readme --features address20
//!
//! # On Aptos
//! cargo install move-readme --features address32
//! ```
//!
//! ## Usage
//!
//! In a directory containing a `Move.toml`, run:
//!
//! ```bash
//! move-readme
//! ```
//!
//! This will write a `README.md` file to the current directory.
use std::path::PathBuf;

use anyhow::*;
use clitool::CliTool;
use move_readme_generator::generate_readme;

/// Parses a Move workspace into a set of IDLs.
#[derive(clap::Parser)]
#[clap(name = "move-readme", author, version)]
pub struct MoveReadmeTool {
    /// Path to the root of the Move workspace.
    #[clap(default_value = ".")]
    pub root: PathBuf,
    /// Path to the output file.
    #[clap(short, long, default_value = "README.md")]
    pub outfile: PathBuf,
}

#[async_trait::async_trait]
impl CliTool<()> for MoveReadmeTool {
    async fn execute(self) -> Result<()> {
        let readme = generate_readme(&self.root)?;
        std::fs::write(self.outfile, readme)?;
        Ok(())
    }
}
