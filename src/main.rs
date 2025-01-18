use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use linking::link_file;
use tempfile::tempdir;

mod cli;
mod codegen;
mod linking;
mod optimisations;
mod parsing;
mod preopts;
mod rust_output;

fn main() {
    let args = cli::Args::parse();

    let x = BufReader::new(File::open(&args.input).unwrap());
    let y = parsing::Token::parse(x.bytes());

    let preopts_result = preopts::process(y).unwrap();
    let preopt_node_count = preopts_result.len();
    let midopts_result = optimisations::convert_nodes(preopts_result);

    println!(
        "Reduced node count from {} to {} ({} nodes reduced)",
        preopt_node_count,
        midopts_result.len(),
        preopt_node_count - midopts_result.len()
    );

    if let Some(rust_path) = args.rust {
        rust_output::write_rust_code(&midopts_result, rust_path);
    }

    if args.object {
        let obj_file = args
            .output
            .unwrap_or_else(|| args.input.with_extension("o"));

        codegen::generate(midopts_result.into_iter(), &obj_file, args.clif).unwrap();
    } else {
        let temp_dir = tempdir().unwrap();
        let obj_file = temp_dir.path().join("out.o");
        let output_path = args.output.unwrap_or_else(|| args.input.with_extension(""));

        codegen::generate(midopts_result.into_iter(), &obj_file, args.clif).unwrap();
        link_file(&obj_file, &output_path);
    }
}
