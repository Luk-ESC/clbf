use std::sync::mpsc::Sender;

use crate::parsing::Token;

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

#[derive(Debug, PartialEq, Eq)]
pub enum IrNode {
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
    sender: Sender<IrNode>,
) -> std::io::Result<()> {
    let mut repeating = repetition::Repeating::new();
    // TODO: Implement optimizations
    for token in tokens {
        let token = token?;

        if let Some(ir_node) = repeating.add(token) {
            sender.send(ir_node).unwrap();
        }

        match token {
            Token::Dot => sender.send(IrNode::PrintChar).unwrap(),
            Token::Comma => sender.send(IrNode::ReadChar).unwrap(),
            Token::OpenBracket => {
                sender.send(IrNode::LoopStart).unwrap();
            }
            Token::CloseBracket => {
                sender.send(IrNode::LoopEnd).unwrap();
            }
            _ => {}
        }
    }

    if let Some(ir_node) = repeating.to_ir_node() {
        sender.send(ir_node).unwrap();
    }
    Ok(())
}
