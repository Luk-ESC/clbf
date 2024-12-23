use crate::parsing::Token;

mod repetition {
    use crate::parsing::Token;

    use super::SimpleIrNode;

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

        pub fn add(&mut self, token: Token) -> Option<SimpleIrNode> {
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

        pub fn to_ir_node(self) -> Option<SimpleIrNode> {
            match self {
                Repeating::Value(x) => Some(SimpleIrNode::ChangeValue(x)),
                Repeating::Ptr(x) => Some(SimpleIrNode::ChangePtr(x)),
                Repeating::None => None,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SimpleIrNode {
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

pub fn process(
    tokens: impl Iterator<Item = std::io::Result<Token>>,
) -> std::io::Result<Vec<SimpleIrNode>> {
    let mut repeating = repetition::Repeating::new();
    let mut nodes = Vec::new();
    // TODO: Implement optimizations
    for token in tokens {
        let token = token?;

        if let Some(ir_node) = repeating.add(token) {
            nodes.push(ir_node);
        }

        match token {
            Token::Dot => nodes.push(SimpleIrNode::PrintChar),
            Token::Comma => nodes.push(SimpleIrNode::ReadChar),
            Token::OpenBracket => {
                nodes.push(SimpleIrNode::LoopStart);
            }
            Token::CloseBracket => {
                nodes.push(SimpleIrNode::LoopEnd);
            }
            _ => {}
        }
    }

    if let Some(ir_node) = repeating.to_ir_node() {
        nodes.push(ir_node);
    }
    Ok(nodes)
}
