#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Left = b'<',
    Right = b'>',
    Plus = b'+',
    Minus = b'-',
    Dot = b'.',
    Comma = b',',
    OpenBracket = b'[',
    CloseBracket = b']',
}

impl Token {
    fn from_byte(c: u8) -> Option<Token> {
        match c {
            b'<' => Some(Token::Left),
            b'>' => Some(Token::Right),
            b'+' => Some(Token::Plus),
            b'-' => Some(Token::Minus),
            b'.' => Some(Token::Dot),
            b',' => Some(Token::Comma),
            b'[' => Some(Token::OpenBracket),
            b']' => Some(Token::CloseBracket),
            _ => None,
        }
    }

    pub fn parse(
        input: impl Iterator<Item = std::io::Result<u8>>,
    ) -> impl Iterator<Item = std::io::Result<Token>> {
        input.filter_map(|x| match x {
            Ok(v) => Token::from_byte(v).map(Ok),
            Err(e) => Some(Err(e)),
        })
    }
}
