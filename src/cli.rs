use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "solang-soroban-companion")]
#[command(about = "A developer companion tool for Solang-compiled Soroban contracts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Inspect a compiled Soroban .wasm file
    Inspect {
        /// The path to the .wasm file
        #[arg(short, long)]
        path: PathBuf,
    },
    /// Generate Rust test boilerplate for the contract
    GenTest {
        /// The path to the .wasm file
        #[arg(short, long)]
        path: PathBuf,
    },
}
