use std::{
    cmp::Ordering,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
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
    let preopt_node_count = preopts_result.len();
    let midopts_result = midopts::convert_nodes(preopts_result);

    println!(
        "Reduced node count from {} to {} ({} nodes reduced)",
        preopt_node_count,
        midopts_result.len(),
        preopt_node_count - midopts_result.len()
    );

    let mut debug = BufWriter::new(File::create("bin/debug.txt").unwrap());
    let mut indent = String::from("\t");
    writeln!(&mut debug, "fn main() {{").unwrap();
    writeln!(&mut debug, "\tlet mut grid = [0u8; 30000];").unwrap();
    writeln!(&mut debug, "\tlet mut ptr = 0;").unwrap();
    writeln!(&mut debug).unwrap();

    for i in midopts_result.iter() {
        match i {
            midopts::IrNode::SetValue(x, offset) => {
                let offset = match offset.cmp(&0) {
                    Ordering::Less => format!(" - {}", offset.abs()),
                    Ordering::Equal => String::new(),
                    Ordering::Greater => format!(" + {offset}"),
                };

                writeln!(&mut debug, "{}grid[ptr{offset}] = {};", indent, x).unwrap();
            }
            midopts::IrNode::ChangeValue(x, offset) => {
                let offset = match offset.cmp(&0) {
                    Ordering::Less => format!(" - {}", offset.abs()),
                    Ordering::Equal => String::new(),
                    Ordering::Greater => format!(" + {offset}"),
                };

                if *x < 0 {
                    writeln!(&mut debug, "{}grid[ptr{offset}] -= {};", indent, x.abs()).unwrap();
                } else {
                    writeln!(&mut debug, "{}grid[ptr{offset}] += {};", indent, x).unwrap();
                }
            }
            midopts::IrNode::ChangePtr(p) => {
                if *p < 0 {
                    writeln!(&mut debug, "{}ptr -= {};", indent, p.abs()).unwrap();
                } else {
                    writeln!(&mut debug, "{}ptr += {};", indent, p).unwrap();
                }
            }
            midopts::IrNode::PrintChar => {
                writeln!(&mut debug, "{}print!(\"{{}}\", grid[ptr] as char);", indent).unwrap();
            }
            midopts::IrNode::ReadChar => {
                writeln!(
                    &mut debug,
                    "{}grid[ptr] = std::io::stdin().bytes().next().unwrap().unwrap();",
                    indent
                )
                .unwrap();
            }
            midopts::IrNode::LoopStart => {
                writeln!(&mut debug).unwrap();
                writeln!(&mut debug, "{}while grid[ptr] != 0 {{", indent).unwrap();
                indent = format!("{}\t", indent);
            }
            midopts::IrNode::LoopEnd => {
                indent = indent.chars().skip(1).collect();
                writeln!(&mut debug, "{}}}", indent).unwrap();
                writeln!(&mut debug).unwrap();
            }
        }
    }

    writeln!(&mut debug, "}}").unwrap();

    codegen::generate(midopts_result.into_iter(), args.output).unwrap();
}
