use std::{fmt::Display, str::FromStr};

pub const SPECIAL_START_CHARS: [char; 1] = ['_'];

#[derive(Debug, PartialEq, Clone)]
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

impl Keyword {
    pub fn name(&self) -> &'static str {
        match self {
            Keyword::And => "AND",
            Keyword::Class => "CLASS",
            Keyword::Else => "ELSE",
            Keyword::False => "FALSE",
            Keyword::For => "FOR",
            Keyword::Fun => "FUN",
            Keyword::If => "IF",
            Keyword::Nil => "NIL",
            Keyword::Or => "OR",
            Keyword::Print => "PRINT",
            Keyword::Return => "RETURN",
            Keyword::Super => "SUPER",
            Keyword::This => "THIS",
            Keyword::True => "TRUE",
            Keyword::Var => "VAR",
            Keyword::While => "WHILE",
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_lowercase())
    }
}

impl FromStr for Keyword {
    type Err = &'static str;

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
            _ => Err("Invalid keyword"),
        }
    }
}
