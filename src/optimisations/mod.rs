mod clear;
mod driveby;
mod inline;

#[derive(Debug, PartialEq, Eq)]
pub enum IrNode {
    // Set the value at the current pointer + offset to the given value
    SetValue(u8, i32),

    // Change the value at the current pointer + offset by the given amount
    ChangeValue(i32, i32),

    // Change the value at the current pointer + offset by the given amount * current value
    DynamicChangeValue(i32, i32),

    // Move the pointer to by the given amount
    ChangePtr(i32),

    // Print the value at the current pointer
    PrintChar,

    // Print the value from the current pointer to pointer + n (inclusive)
    //Print(NonZeroU32),

    // Read a character from stdin and store it at the current pointer
    ReadChar,

    // Read n characters from stdin and store them at the current pointer to pointer + n (inclusive)
    //Read(NonZeroU32),

    // Start a loop
    LoopStart,

    // End the loop
    LoopEnd,
}

pub(crate) fn convert_nodes(mut nodes: Vec<IrNode>) -> Vec<IrNode> {
    clear::clear(&mut nodes);
    driveby::driveby(&mut nodes);
    inline::inline(&mut nodes);

    nodes
}
