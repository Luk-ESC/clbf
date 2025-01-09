mod clear;

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

    // Search for the following pattern:
    // ChangePtr(x), SetValue | ChangeValue, ChangePtr(-x)
    // Replace with SetValue | ChangeValue and adjusted offset
    let mut to_replace = vec![];
    for (i, window) in nodes.windows(3).enumerate() {
        match window {
            [IrNode::ChangePtr(x), IrNode::SetValue(value, offset), IrNode::ChangePtr(y)]
                if (x.signum() != y.signum()) =>
            {
                assert!(*x != 0 && *y != 0);
                to_replace.push((true, *value as i32, (offset + x), i, x + y))
            }

            [IrNode::ChangePtr(x), IrNode::ChangeValue(value, offset), IrNode::ChangePtr(y)]
                if (x.signum() != y.signum()) =>
            {
                assert!(*x != 0 && *y != 0);
                to_replace.push((false, *value, (offset + x), i, x + y))
            }

            _ => {}
        }
    }

    for (set, value, offset, start_idx, after_offset) in to_replace.into_iter().rev() {
        if !matches!(
            &nodes[start_idx..start_idx + 3],
            &[
                IrNode::ChangePtr(x),
                IrNode::ChangeValue(_, _) | IrNode::SetValue(_, _),
                IrNode::ChangePtr(y)
                ] if (x.signum() != y.signum())
        ) {
            // TODO: If you have CP CV CP CV CP, they overwrite each other............
            // Maybe just run optim again?
            continue;
        }

        nodes[start_idx] = if set {
            assert!((0..256).contains(&value));
            IrNode::SetValue(value as u8, offset)
        } else {
            IrNode::ChangeValue(value, offset)
        };

        nodes.remove(start_idx + 2);
        nodes[start_idx + 1] = IrNode::ChangePtr(after_offset);
    }

    // Search for the following pattern:
    // StartLoop ChangeValue(-1, 0) (ChangeValue(_, !0) | SetValue(_, !0))* EndLoop
    // Replace With:
    // (DynamicChangeValue(_, !0) | SetValue(_, !0))* SetValue(0, 0)
    // TODO: This optimisation does not actually hold... idk i like it anyways
    let mut iter = nodes.iter().enumerate();
    let mut to_replace = vec![];
    while let Some((mut loop_start_i, loop_start)) = iter.next() {
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
                // LoopStart can't be part of multiplication, so let's reset everything and start a new search
                IrNode::LoopStart => {
                    do_optimisation = true;
                    change_to_this = 0;
                    loop_end = 0;
                    loop_start_i = i;
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

        assert!(loop_start_i < loop_end);
        assert!(nodes[loop_start_i] == IrNode::LoopStart);
        to_replace.push((loop_start_i, loop_end));
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
