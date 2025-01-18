use std::path::PathBuf;

use clap::Parser;

/// BF Compiler with cranelift backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg()]
    pub input: PathBuf,

    /// Output file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output object file
    #[arg(long)]
    pub object: bool,

    /// Rust output file
    #[arg(short, long)]
    pub rust: Option<PathBuf>,

    /// Clif output file
    #[arg(short, long)]
    pub clif: Option<PathBuf>,
}
