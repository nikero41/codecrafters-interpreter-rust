use std::fmt::Display;

use crate::token::Keyword;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    SemiColon,
    Minus,
    Slash,
    Equal,
    Assign,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    String(String),
    Number(String),
    Identifier(String),
    Keyword(Keyword),
    Eof,
    Dummy,
}

impl TokenType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",
            Self::Star => "STAR",
            Self::Dot => "DOT",
            Self::Comma => "COMMA",
            Self::Plus => "PLUS",
            Self::SemiColon => "SEMICOLON",
            Self::Minus => "MINUS",
            Self::Slash => "SLASH",
            Self::Equal => "EQUAL_EQUAL",
            Self::Assign => "EQUAL",
            Self::Bang => "BANG",
            Self::BangEqual => "BANG_EQUAL",
            Self::Less => "LESS",
            Self::LessEqual => "LESS_EQUAL",
            Self::Greater => "GREATER",
            Self::GreaterEqual => "GREATER_EQUAL",
            Self::String(_) => "STRING",
            Self::Number(_) => "NUMBER",
            Self::Identifier(_) => "IDENTIFIER",
            Self::Keyword(keyword) => keyword.name(),
            Self::Eof => "EOF",
            Self::Dummy => "Dummy",
        }
    }

    // TODO: examine using Cow
    pub fn lexeme(&self) -> String {
        match self {
            Self::LeftParen => "(".to_string(),
            Self::RightParen => ")".to_string(),
            Self::LeftBrace => "{".to_string(),
            Self::RightBrace => "}".to_string(),
            Self::Star => "*".to_string(),
            Self::Dot => ".".to_string(),
            Self::Comma => ",".to_string(),
            Self::Plus => "+".to_string(),
            Self::SemiColon => ";".to_string(),
            Self::Minus => "-".to_string(),
            Self::Slash => "/".to_string(),
            Self::Equal => "==".to_string(),
            Self::Assign => "=".to_string(),
            Self::Bang => "!".to_string(),
            Self::BangEqual => "!=".to_string(),
            Self::Less => "<".to_string(),
            Self::LessEqual => "<=".to_string(),
            Self::Greater => ">".to_string(),
            Self::GreaterEqual => ">=".to_string(),
            Self::String(literal) => format!(r#""{}""#, literal),
            Self::Number(float) => float.to_string(),
            Self::Identifier(identifier) => identifier.to_string(),
            Self::Keyword(keyword) => format!("{}", keyword),
            Self::Eof => String::new(),
            Self::Dummy => "DUMMY".to_string(),
        }
    }

    // TODO: examine using Cow
    pub fn literal(&self) -> String {
        match self {
            Self::String(literal) => literal.clone(),
            Self::Number(float) => format!("{:?}", float.parse::<f64>().unwrap()),
            _ => "null".to_string(),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name(), self.lexeme(), self.literal())
    }
}
