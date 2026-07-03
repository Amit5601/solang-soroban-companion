mod cli;
mod generator;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    match &cli.command {
        Commands::Inspect { path } => {
            parser::inspect_wasm_file(path)?;
        }
        Commands::GenTest { path } => {
            generator::generate_test_boilerplate(path)?;
        }
    }

    Ok(())
}
