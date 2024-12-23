#[derive(Debug, PartialEq, Eq)]
pub enum IrNode {
    // Set the value at the current pointer to the given value
    SetValue(i32),

    // Change the value at the current pointer by the given amount
    ChangeValue(i32),

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
    for i in (0..nodes.len()).rev() {
        if matches!(nodes[i], IrNode::ChangeValue(0)) {
            nodes.remove(i);
        }

        if matches!(nodes[i], IrNode::ChangePtr(0)) {
            nodes.remove(i);
        }
    }

    // Search for the following pattern:
    // ChangeValue(_)? LoopStart ChangeValue(-x) LoopEnd ChangeValue(x)
    // If found replace with SetValue(x)

    let mut to_replace = Vec::new();
    for (first_index, window) in nodes.windows(5).enumerate() {
        match &window {
            &[b, IrNode::LoopStart, IrNode::ChangeValue(x), IrNode::LoopEnd, l] if *x < 0 => {
                let y = match l {
                    IrNode::ChangeValue(y) => Some(*y),
                    _ => None,
                };

                let remove_first = matches!(b, IrNode::ChangeValue(_));

                to_replace.push((first_index, y, remove_first));
            }

            _ => {}
        }
    }

    for (index, value, remove_first) in to_replace.into_iter().rev() {
        if let Some(value) = value {
            nodes[index + 1] = IrNode::SetValue(value);
            nodes.remove(index + 2);
            nodes.remove(index + 2);
            nodes.remove(index + 2);
        } else {
            nodes[index + 1] = IrNode::SetValue(0);
            nodes.remove(index + 2);
            nodes.remove(index + 2);
        }

        if remove_first {
            nodes.remove(index);
        }
    }

    nodes
}
