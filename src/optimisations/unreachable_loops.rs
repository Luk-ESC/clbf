use super::IrNode;

pub fn unreachable_loops(nodes: &mut Vec<IrNode>) {
    if nodes.first() == Some(&IrNode::LoopStart) {
        let mut depth = 0;
        let mut i = 1;

        for n in &nodes[1..] {
            match n {
                IrNode::LoopEnd => {
                    if depth == 0 {
                        break;
                    }

                    depth -= 1;
                }

                IrNode::LoopStart => depth += 1,

                _ => {}
            }
            i += 1;
        }

        nodes.drain(0..=i);
    }
}
