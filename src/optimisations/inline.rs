use super::IrNode;

struct Match {
    start_index: usize,
    end_index: usize,
    changes: Vec<(i32, i32, bool)>,
}

impl Match {
    fn collect_all(nodes: &[IrNode]) -> Vec<Self> {
        let mut matches = vec![];
        // Reused across iterations
        let mut current_changes = vec![];

        let mut start_index = 1;
        while start_index < nodes.len() {
            if nodes[start_index - 1] != IrNode::LoopStart {
                start_index += 1;
                continue;
            }

            let mut end_index = start_index;

            let mut ptr_offset = 0;
            let mut change_to_this = 0;
            for i in &nodes[start_index..] {
                match i {
                    IrNode::ChangeValue(v, o) => {
                        if *o == 0 {
                            change_to_this += *v;
                        } else {
                            current_changes.push((*v, o + ptr_offset, false))
                        }
                    }
                    IrNode::ChangePtr(ptr) => {
                        ptr_offset += *ptr;
                    }
                    _ => break,
                }

                end_index += 1;
            }

            if ptr_offset == 0 && nodes[end_index] == IrNode::LoopEnd && change_to_this == -1 {
                matches.push(Match {
                    start_index: start_index - 1,
                    end_index,
                    changes: current_changes.clone(),
                });
            }

            current_changes.clear();
            start_index = end_index + 1;
        }
        matches
    }

    fn apply(self, nodes: &mut Vec<IrNode>) {
        let Self {
            start_index,
            end_index,
            changes,
        } = self;

        assert_eq!(nodes[start_index], IrNode::LoopStart);
        assert_eq!(nodes[end_index], IrNode::LoopEnd);
        let mut i = start_index;

        for (v, o, set) in changes.iter().copied() {
            if set {
                assert!((0..=255).contains(&v));
                nodes[i] = IrNode::SetValue(v as u8, o);
            } else {
                nodes[i] = IrNode::DynamicChangeValue(v, o);
            }

            i += 1;
        }

        nodes[i] = IrNode::SetValue(0, 0);
        i += 1;

        assert!(i <= end_index);
        for _ in i..=end_index {
            nodes.remove(i);
        }
    }
}

pub fn inline(nodes: &mut Vec<IrNode>) {
    let matches = Match::collect_all(nodes);

    for i in matches.into_iter().rev() {
        i.apply(nodes);
    }
}
