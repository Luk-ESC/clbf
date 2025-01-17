mod clear;
mod driveby;
mod inline;

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

    // Print the value at the current pointer
    PrintChar,

    // Read a character from stdin and store it at the current pointer
    ReadChar,

    // Start a loop
    LoopStart,

    // End the loop
    LoopEnd,
}

pub(crate) fn convert_nodes(mut nodes: Vec<IrNode>) -> Vec<IrNode> {
    clear::clear(&mut nodes);
    driveby::driveby(&mut nodes);
    inline::inline(&mut nodes);
    driveby::driveby(&mut nodes);

    nodes
}
