use std::{
    fs::File,
    io::{BufReader, Read},
};

mod codegen;
mod parsing;
mod preopts;

fn main() {
    let x = BufReader::new(File::open("test.bf").unwrap());
    let y = parsing::Token::parse(x.bytes());
    let (sender, receiver) = std::sync::mpsc::channel();

    preopts::process(y, sender).unwrap();
    codegen::generate(receiver);
}
