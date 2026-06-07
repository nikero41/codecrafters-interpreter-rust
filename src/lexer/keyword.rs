use std::{fmt::Display, str::FromStr};

pub const SPECIAL_START_CHARS: [char; 1] = ['_'];

#[derive(Debug)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::And => todo!(),
            Keyword::Class => todo!(),
            Keyword::Else => todo!(),
            Keyword::False => todo!(),
            Keyword::For => todo!(),
            Keyword::Fun => todo!(),
            Keyword::If => todo!(),
            Keyword::Nil => todo!(),
            Keyword::Or => todo!(),
            Keyword::Print => todo!(),
            Keyword::Return => todo!(),
            Keyword::Super => todo!(),
            Keyword::This => todo!(),
            Keyword::True => todo!(),
            Keyword::Var => todo!(),
            Keyword::While => todo!(),
        }
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(Keyword::And),
            "class" => Ok(Keyword::Class),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "for" => Ok(Keyword::For),
            "fun" => Ok(Keyword::Fun),
            "if" => Ok(Keyword::If),
            "nil" => Ok(Keyword::Nil),
            "or" => Ok(Keyword::Or),
            "print" => Ok(Keyword::Print),
            "return" => Ok(Keyword::Return),
            "super" => Ok(Keyword::Super),
            "this" => Ok(Keyword::This),
            "true" => Ok(Keyword::True),
            "var" => Ok(Keyword::Var),
            "while" => Ok(Keyword::While),
            _ => Err(()),
        }
    }
}
