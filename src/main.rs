use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;

mod cli;
mod codegen;
mod midopts;
mod parsing;
mod preopts;

fn main() {
    let args = cli::Args::parse();

    let x = BufReader::new(File::open(args.input).unwrap());
    let y = parsing::Token::parse(x.bytes());

    let preopts_result = preopts::process(y).unwrap();
    let midopts_result = midopts::convert_nodes(preopts_result);

    codegen::generate(midopts_result.into_iter(), args.output).unwrap();
}
