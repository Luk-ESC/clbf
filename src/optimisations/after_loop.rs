use super::IrNode;

pub fn after_loop(nodes: &mut [IrNode]) {
    let mut to_replace = vec![];
    let mut i = 1;
    for w in nodes.windows(2) {
        if let [IrNode::LoopEnd, IrNode::ChangeValue(v, o)] = w {
            to_replace.push((i, IrNode::SetValue(*v as u8, *o)))
        }
        i += 1;
    }

    for (i, v) in to_replace {
        nodes[i] = v;
    }
}
