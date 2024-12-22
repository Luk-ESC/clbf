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
    #[arg(short, long, default_value = "a.out")]
    pub output: PathBuf,
}
