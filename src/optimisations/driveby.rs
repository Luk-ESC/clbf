use super::IrNode;

struct Match {
    start_index: usize,
    end_index: usize,
    changes: Vec<(i32, i32, bool)>,
    ptr_offset: i32,
}

impl Match {
    fn collect_all(nodes: &[IrNode]) -> Vec<Self> {
        let mut matches = vec![];
        // Reused across iterations
        let mut current_changes = vec![];

        let mut start_index = 0;
        while start_index < nodes.len() {
            let mut end_index = start_index;

            let mut ptr_offset = 0;
            let mut change_ptrs = 0;
            for i in &nodes[start_index..] {
                match i {
                    IrNode::SetValue(v, o) => {
                        current_changes.push((*v as i32, o + ptr_offset, true))
                    }
                    IrNode::ChangeValue(v, o) => current_changes.push((*v, o + ptr_offset, false)),
                    IrNode::ChangePtr(ptr) => {
                        change_ptrs += 1;
                        ptr_offset += *ptr;
                    }
                    _ => break,
                }

                end_index += 1;
            }

            if change_ptrs > 1 {
                matches.push(Match {
                    start_index,
                    end_index,
                    changes: current_changes.clone(),
                    ptr_offset,
                })
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
            ptr_offset,
        } = self;

        let mut i = start_index;

        for (v, o, set) in changes {
            if set {
                assert!((0..=255).contains(&v));
                nodes[i] = IrNode::SetValue(v as u8, o);
            } else {
                nodes[i] = IrNode::ChangeValue(v, o);
            }

            i += 1;
        }

        if ptr_offset != 0 {
            nodes[i] = IrNode::ChangePtr(ptr_offset);
            i += 1;
        }

        assert!(i < end_index);
        for _ in i..end_index {
            nodes.remove(i);
        }
    }
}

pub fn driveby(nodes: &mut Vec<IrNode>) {
    let matches = Match::collect_all(nodes);
    for i in matches.into_iter().rev() {
        i.apply(nodes);
    }
}