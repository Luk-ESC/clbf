use std::{
    cmp::Ordering,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::optimisations::IrNode;

pub fn write_rust_code(nodes: &[IrNode], out_path: PathBuf) {
    let mut debug = BufWriter::new(File::create(out_path).unwrap());
    let mut indent = String::from("\t");
    writeln!(&mut debug, "fn main() {{").unwrap();
    writeln!(&mut debug, "\tlet mut grid = [0u8; 30000];").unwrap();
    writeln!(&mut debug, "\tlet mut ptr = 0;").unwrap();
    writeln!(&mut debug).unwrap();

    for i in nodes.iter() {
        match i {
            IrNode::SetValue(x, offset) => {
                let offset = match offset.cmp(&0) {
                    Ordering::Less => format!(" - {}", offset.abs()),
                    Ordering::Equal => String::new(),
                    Ordering::Greater => format!(" + {offset}"),
                };

                writeln!(&mut debug, "{}grid[ptr{offset}] = {};", indent, x).unwrap();
            }
            IrNode::ChangeValue(x, offset) => {
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
            IrNode::DynamicChangeValue(-1, offset) => {
                let offset = match offset.cmp(&0) {
                    Ordering::Less => format!(" - {}", offset.abs()),
                    Ordering::Equal => String::new(),
                    Ordering::Greater => format!(" + {offset}"),
                };

                writeln!(&mut debug, "{}grid[ptr{offset}] -= grid[ptr];", indent).unwrap();
            }
            IrNode::DynamicChangeValue(1, offset) => {
                let offset = match offset.cmp(&0) {
                    Ordering::Less => format!(" - {}", offset.abs()),
                    Ordering::Equal => String::new(),
                    Ordering::Greater => format!(" + {offset}"),
                };

                writeln!(&mut debug, "{}grid[ptr{offset}] += grid[ptr];", indent).unwrap();
            }
            IrNode::DynamicChangeValue(x, offset) => {
                let offset = match offset.cmp(&0) {
                    Ordering::Less => format!(" - {}", offset.abs()),
                    Ordering::Equal => String::new(),
                    Ordering::Greater => format!(" + {offset}"),
                };

                if *x < 0 {
                    writeln!(
                        &mut debug,
                        "{}grid[ptr{offset}] -= {} * grid[ptr];",
                        indent,
                        x.abs()
                    )
                    .unwrap();
                } else {
                    writeln!(
                        &mut debug,
                        "{}grid[ptr{offset}] += {} * grid[ptr];",
                        indent, x
                    )
                    .unwrap();
                }
            }
            IrNode::ChangePtr(p) => {
                if *p < 0 {
                    writeln!(&mut debug, "{}ptr -= {};", indent, p.abs()).unwrap();
                } else {
                    writeln!(&mut debug, "{}ptr += {};", indent, p).unwrap();
                }
            }
            IrNode::PrintChar => {
                writeln!(&mut debug, "{}print!(\"{{}}\", grid[ptr] as char);", indent).unwrap();
            }
            IrNode::ReadChar => {
                writeln!(
                    &mut debug,
                    "{}grid[ptr] = std::io::stdin().bytes().next().unwrap().unwrap();",
                    indent
                )
                .unwrap();
            }
            IrNode::LoopStart => {
                writeln!(&mut debug).unwrap();
                writeln!(&mut debug, "{}while grid[ptr] != 0 {{", indent).unwrap();
                indent = format!("{}\t", indent);
            }
            IrNode::LoopEnd => {
                indent = indent.chars().skip(1).collect();
                writeln!(&mut debug, "{}}}", indent).unwrap();
                writeln!(&mut debug).unwrap();
            }
        }
    }

    writeln!(&mut debug, "}}").unwrap();
}
