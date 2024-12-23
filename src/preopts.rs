use std::sync::mpsc::Sender;

use crate::parsing::Token;

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
    mut tokens: impl Iterator<Item = std::io::Result<Token>>,
    sender: Sender<IrNode>,
) -> std::io::Result<()> {
    let mut last_kind = Some(tokens.next().unwrap()?);
    let mut repeat_count = 1;

    // TODO: Implement optimizations
    for token in tokens {
        let token = token?;

        if last_kind == Some(token) {
            repeat_count += 1;
            continue;
        } else if let Some(last_kind) = last_kind {
            // Not equal, but last_kind is Some
            match last_kind {
                Token::Left => sender.send(IrNode::ChangePtr(-repeat_count)).unwrap(),
                Token::Right => sender.send(IrNode::ChangePtr(repeat_count)).unwrap(),
                Token::Plus => sender.send(IrNode::ChangeValue(repeat_count)).unwrap(),
                Token::Minus => sender.send(IrNode::ChangeValue(-repeat_count)).unwrap(),
                e => unreachable!("non repeatable token in last_kind: {e:?}"),
            }

            repeat_count = 1;
        }

        let mut next_last_kind = None;
        match token {
            Token::Dot => sender.send(IrNode::PrintChar).unwrap(),
            Token::Comma => sender.send(IrNode::ReadChar).unwrap(),
            Token::OpenBracket => {
                sender.send(IrNode::LoopStart).unwrap();
            }
            Token::CloseBracket => {
                sender.send(IrNode::LoopEnd).unwrap();
            }
            _ => {
                next_last_kind = Some(token);
            }
        }

        last_kind = next_last_kind;
    }

    if let Some(last_kind) = last_kind {
        match last_kind {
            Token::Left => sender.send(IrNode::ChangePtr(-repeat_count)).unwrap(),
            Token::Right => sender.send(IrNode::ChangePtr(repeat_count)).unwrap(),
            Token::Plus => sender.send(IrNode::ChangeValue(repeat_count)).unwrap(),
            Token::Minus => sender.send(IrNode::ChangeValue(-repeat_count)).unwrap(),
            _ => unreachable!("non repeatable token in last_kind"),
        }
    }

    Ok(())
}
