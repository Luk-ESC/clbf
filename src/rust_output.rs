use std::{
    cmp::Ordering,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::optimisations::IrNode;

fn add(amount: i32) -> String {
    match amount.cmp(&0) {
        Ordering::Less => format!(" - {}", amount.abs()),
        Ordering::Equal => String::new(),
        Ordering::Greater => format!(" + {amount}"),
    }
}

pub fn write_rust_code(nodes: &[IrNode], out_path: PathBuf) {
    let mut debug = BufWriter::new(File::create(out_path).unwrap());
    let mut indent = String::from("\t");

    /*
       unsafe extern "C" {
               fn putchar(char: i32) -> i32;
               fn getchar() -> i32;
       }
    */
    writeln!(&mut debug, "unsafe extern \"C\" {{").unwrap();
    writeln!(&mut debug, "\tsafe fn putchar(ch: i32) -> i32;").unwrap();
    writeln!(&mut debug, "\tsafe fn getchar() -> i32;").unwrap();

    writeln!(&mut debug, "}}\n").unwrap();

    writeln!(&mut debug, "fn main() {{").unwrap();
    writeln!(&mut debug, "\tlet mut grid = [0u8; 30000];").unwrap();
    writeln!(&mut debug, "\tlet mut ptr = 15000;").unwrap();
    writeln!(&mut debug).unwrap();

    for i in nodes.iter() {
        match i {
            IrNode::SetValue(x, offset) => {
                let offset = add(*offset);

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
            IrNode::DynamicChangeValue(-1, offset, mult_offset) => {
                let offset = add(*offset);
                let mult_offset = add(*mult_offset);

                writeln!(
                    &mut debug,
                    "{}grid[ptr{offset}] -= grid[ptr{mult_offset}];",
                    indent
                )
                .unwrap();
            }
            IrNode::DynamicChangeValue(1, offset, mult_offset) => {
                let offset = add(*offset);
                let mult_offset = add(*mult_offset);

                writeln!(
                    &mut debug,
                    "{}grid[ptr{offset}] += grid[ptr{mult_offset}];",
                    indent
                )
                .unwrap();
            }
            IrNode::DynamicChangeValue(x, offset, mult_offset) => {
                let offset = add(*offset);
                let mult_offset = add(*mult_offset);
                println!("{mult_offset}");
                if *x < 0 {
                    writeln!(
                        &mut debug,
                        "{}grid[ptr{offset}] -= {} * grid[ptr{mult_offset}];",
                        indent,
                        x.abs()
                    )
                    .unwrap();
                } else {
                    writeln!(
                        &mut debug,
                        "{}grid[ptr{offset}] += {} * grid[ptr{mult_offset}];",
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
            IrNode::PrintChar(offset) => {
                let offset = add(*offset);

                writeln!(&mut debug, "{}putchar(grid[ptr{offset}] as i32);", indent).unwrap();
            }
            IrNode::ReadChar => {
                writeln!(&mut debug, "{}grid[ptr] = getchar() as u8;", indent).unwrap();
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
