#[derive(Debug, PartialEq, Eq)]
pub enum IrNode {
    // Set the value at the current pointer + offset to the given value
    SetValue(i32, i32),

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
    // Remove ChangeValue(0) and ChangePtr(0), since they don't have any effect.
    for i in (0..nodes.len()).rev() {
        if matches!(nodes[i], IrNode::ChangeValue(0, _)) {
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
            // TODO: Can we use offset here?
            &[b, IrNode::LoopStart, IrNode::ChangeValue(x, 0), IrNode::LoopEnd, l] if *x < 0 => {
                // Make sure offset is 0
                if matches!(l, IrNode::ChangeValue(_, x) if *x != 0) {
                    continue;
                }

                let y = match l {
                    IrNode::ChangeValue(y, 0) => Some(*y),
                    _ => None,
                };

                let remove_first = matches!(b, IrNode::ChangeValue(_, 0));

                to_replace.push((first_index, y, remove_first));
            }

            _ => {}
        }
    }

    for (index, value, remove_first) in to_replace.into_iter().rev() {
        if let Some(value) = value {
            nodes[index + 1] = IrNode::SetValue(value, 0);
            nodes.remove(index + 2);
            nodes.remove(index + 2);
            nodes.remove(index + 2);
        } else {
            nodes[index + 1] = IrNode::SetValue(0, 0);
            nodes.remove(index + 2);
            nodes.remove(index + 2);
        }

        if remove_first {
            nodes.remove(index);
        }
    }

    // Search for the following pattern:
    // ChangePtr(x), SetValue | ChangeValue, ChangePtr(-x)
    // Replace with SetValue | ChangeValue and adjusted offset
    let mut to_replace = vec![];
    for (i, window) in nodes.windows(3).enumerate() {
        match window {
            [IrNode::ChangePtr(x), IrNode::SetValue(value, offset), IrNode::ChangePtr(y)]
                if (*x == -y) =>
            {
                to_replace.push((true, *value, (offset + x), i))
            }

            [IrNode::ChangePtr(x), IrNode::ChangeValue(value, offset), IrNode::ChangePtr(y)]
                if (*x == -y) =>
            {
                to_replace.push((false, *value, (offset + x), i))
            }

            _ => {}
        }
    }

    for (set, value, offset, start_idx) in to_replace.into_iter().rev() {
        nodes[start_idx] = if set {
            IrNode::SetValue(value, offset)
        } else {
            IrNode::ChangeValue(value, offset)
        };

        nodes.remove(start_idx + 1);
        nodes.remove(start_idx + 1);
    }

    // Search for the following pattern:
    // StartLoop ChangeValue(-1, 0) (ChangeValue(_, !0) | SetValue(_, !0))* EndLoop
    // Replace With:
    // (DynamicChangeValue(_, !0) | SetValue(_, !0))* SetValue(0, 0)
    // TODO: This optimisation does not actually hold... idk i like it anyways
    let mut iter = nodes.iter().enumerate();
    let mut to_replace = vec![];
    while let Some((i, loop_start)) = iter.next() {
        if !matches!(loop_start, IrNode::LoopStart) {
            continue;
        }

        let mut do_optimisation = true;
        let mut change_to_this = 0;
        let mut loop_end = 0;
        for (i, x) in iter.by_ref() {
            match x {
                IrNode::SetValue(_, offset) => {
                    if *offset == 0 {
                        do_optimisation = false;
                        break;
                    }
                }
                IrNode::ChangeValue(v, offset) => {
                    if *offset == 0 {
                        change_to_this += v;
                    }
                }
                IrNode::LoopEnd => {
                    loop_end = i;
                    break;
                }
                _ => {
                    do_optimisation = false;
                    break;
                }
            }
        }

        if !do_optimisation || change_to_this != -1 {
            continue;
        }

        assert!(i < loop_end);
        assert!(nodes[i] == IrNode::LoopStart);
        to_replace.push((i, loop_end));
    }

    for (start, end) in to_replace.into_iter().rev() {
        nodes[end] = IrNode::SetValue(0, 0);
        nodes.remove(start);

        let mut to_remove = vec![];
        for (i, node) in nodes[start..end - 1].iter_mut().enumerate() {
            *node = match &node {
                IrNode::ChangeValue(_, 0) => {
                    to_remove.push(start + i);
                    continue;
                }
                IrNode::SetValue(v, o) => IrNode::SetValue(*v, *o),
                IrNode::ChangeValue(v, o) => IrNode::DynamicChangeValue(*v, *o),
                e => unreachable!("Node {e:?} doesn't make sense here"),
            }
        }

        for i in to_remove {
            nodes.remove(i);
        }
    }

    nodes
}
