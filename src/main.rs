use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;

mod cli;
mod codegen;
mod parsing;
mod preopts;

fn main() {
    let args = cli::Args::parse();

    let x = BufReader::new(File::open(args.input).unwrap());
    let y = parsing::Token::parse(x.bytes());
    let (sender, receiver) = std::sync::mpsc::channel();

    preopts::process(y, sender).unwrap();
    codegen::generate(receiver, args.output).unwrap();
}
