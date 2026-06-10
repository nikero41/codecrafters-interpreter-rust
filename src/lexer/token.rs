use std::fmt::Display;

use crate::lexer::keyword::Keyword;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    column: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl TokenType {
    pub fn to_token(self, line: u32, column: u32) -> Token {
        Token {
            token_type: self,
            line,
            column,
        }
    }
}

#[derive(Debug, PartialEq)]
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
    EOF,
}

impl TokenType {
    pub fn name(&self) -> String {
        match self {
            Self::LeftParen => "LEFT_PAREN".to_string(),
            Self::RightParen => "RIGHT_PAREN".to_string(),
            Self::LeftBrace => "LEFT_BRACE".to_string(),
            Self::RightBrace => "RIGHT_BRACE".to_string(),
            Self::Star => "STAR".to_string(),
            Self::Dot => "DOT".to_string(),
            Self::Comma => "COMMA".to_string(),
            Self::Plus => "PLUS".to_string(),
            Self::SemiColon => "SEMICOLON".to_string(),
            Self::Minus => "MINUS".to_string(),
            Self::Slash => "SLASH".to_string(),
            Self::Equal => "EQUAL_EQUAL".to_string(),
            Self::Assign => "EQUAL".to_string(),
            Self::Bang => "BANG".to_string(),
            Self::BangEqual => "BANG_EQUAL".to_string(),
            Self::Less => "LESS".to_string(),
            Self::LessEqual => "LESS_EQUAL".to_string(),
            Self::Greater => "GREATER".to_string(),
            Self::GreaterEqual => "GREATER_EQUAL".to_string(),
            Self::String(_) => "STRING".to_string(),
            Self::Number(_) => "NUMBER".to_string(),
            Self::Identifier(_) => "IDENTIFIER".to_string(),
            Self::Keyword(keyword) => format!("{}", keyword).to_uppercase(),
            Self::EOF => "EOF".to_string(),
        }
    }

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
            Self::EOF => String::new(),
        }
    }

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
