use crate::{midopts::IrNode, parsing::Token};

mod repetition {
    use crate::parsing::Token;

    use super::IrNode;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Repeating {
        Value(i32),
        Ptr(i32),
        None,
    }

    impl Repeating {
        pub fn new() -> Self {
            Repeating::None
        }

        pub fn add(&mut self, token: Token) -> Option<IrNode> {
            let old = self.to_ir_node();
            let (change_amount, is_ptr) = match token {
                Token::Left => (-1, true),
                Token::Right => (1, true),
                Token::Minus => (-1, false),
                Token::Plus => (1, false),
                _ => {
                    *self = Repeating::None;
                    return old;
                }
            };

            match self {
                Repeating::Value(v) => {
                    if is_ptr {
                        *self = Repeating::Ptr(change_amount);
                        return old;
                    }

                    *v += change_amount;
                }
                Repeating::Ptr(p) => {
                    if !is_ptr {
                        *self = Repeating::Value(change_amount);
                        return old;
                    }

                    *p += change_amount;
                }
                Repeating::None => {
                    if is_ptr {
                        *self = Repeating::Ptr(change_amount);
                    } else {
                        *self = Repeating::Value(change_amount);
                    }
                }
            }
            None
        }

        pub fn to_ir_node(self) -> Option<IrNode> {
            match self {
                Repeating::Value(x) => Some(IrNode::ChangeValue(x)),
                Repeating::Ptr(x) => Some(IrNode::ChangePtr(x)),
                Repeating::None => None,
            }
        }
    }
}

pub fn process(
    tokens: impl Iterator<Item = std::io::Result<Token>>,
) -> std::io::Result<Vec<IrNode>> {
    let mut repeating = repetition::Repeating::new();
    let mut nodes = Vec::new();
    // TODO: Implement optimizations
    for token in tokens {
        let token = token?;

        if let Some(ir_node) = repeating.add(token) {
            nodes.push(ir_node);
        }

        match token {
            Token::Dot => nodes.push(IrNode::PrintChar),
            Token::Comma => nodes.push(IrNode::ReadChar),
            Token::OpenBracket => {
                nodes.push(IrNode::LoopStart);
            }
            Token::CloseBracket => {
                nodes.push(IrNode::LoopEnd);
            }
            _ => {}
        }
    }

    if let Some(ir_node) = repeating.to_ir_node() {
        nodes.push(ir_node);
    }
    Ok(nodes)
}
