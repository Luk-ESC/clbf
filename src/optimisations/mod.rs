mod after_loop;
mod clear;
mod driveby;
mod inline;
mod unreachable_loops;

#[derive(Debug, PartialEq, Eq)]
pub enum IrNode {
    // Set the value at the current pointer + offset to the given value
    SetValue(u8, i32),

    // Change the value at the current pointer + offset by the given amount
    ChangeValue(i32, i32),

    // Change the value at the current pointer + offset by the given amount * value at the second_offset
    DynamicChangeValue(i32, i32, i32),

    // Move the pointer to by the given amount
    ChangePtr(i32),

    // Print the value at the current pointer + offset
    PrintChar(i32),

    // Read a character from stdin and store it at the current pointer
    ReadChar,

    // Start a loop
    LoopStart,

    // End the loop
    LoopEnd,
}

pub(crate) fn convert_nodes(mut nodes: Vec<IrNode>) -> Vec<IrNode> {
    after_loop::after_loop(&mut nodes);
    unreachable_loops::unreachable_loops(&mut nodes);
    clear::clear(&mut nodes);
    driveby::driveby(&mut nodes);
    inline::inline(&mut nodes);
    driveby::driveby(&mut nodes);

    nodes
}
