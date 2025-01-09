use super::IrNode;

struct Match {
    index: usize,
}

impl Match {
    fn collect_all(nodes: &[IrNode]) -> Vec<Self> {
        // Search for the following pattern:
        // LoopStart ChangeValue(_) LoopEnd
        let mut matches = vec![];

        for (index, window) in nodes.windows(3).enumerate() {
            if let &[IrNode::LoopStart, IrNode::ChangeValue(x, 0), IrNode::LoopEnd] = &window {
                // If ChangeValue is 0, this is actually an infinite loop.
                if *x != 0 {
                    matches.push(Match { index });
                }
            }
        }

        matches
    }

    fn apply(self, nodes: &mut Vec<IrNode>) {
        // Replace LoopStart ChangeValue(_) LoopEnd with SetValue(0)
        let Self { index } = self;
        nodes[index] = IrNode::SetValue(0, 0);
        nodes.remove(index + 1);
        nodes.remove(index + 1);

        // SetValue(0) ChangeValue(x) -> SetValue(x)
        if let Some(IrNode::ChangeValue(x, 0)) = nodes.get(index + 1) {
            nodes[index] = IrNode::SetValue(*x as u8, 0);
            nodes.remove(index + 1);
        }

        // ChangeValue(_) SetValue(x) -> SetValue(x)
        if let Some(IrNode::ChangeValue(_, 0)) = nodes.get(index - 1) {
            nodes.remove(index - 1);
        }
    }
}

pub fn clear(nodes: &mut Vec<IrNode>) {
    let matches = Match::collect_all(nodes);
    for i in matches.into_iter().rev() {
        i.apply(nodes);
    }
}
