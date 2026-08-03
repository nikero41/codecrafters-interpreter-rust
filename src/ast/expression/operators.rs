use derive_more::Display;

use crate::token::{Keyword, TokenType};

// operator → "and" | "or" ;
#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

impl TryFrom<&TokenType> for LogicalOp {
    type Error = &'static str;

    fn try_from(token_type: &TokenType) -> Result<Self, Self::Error> {
        match token_type {
            TokenType::Keyword(Keyword::And) => Ok(LogicalOp::And),
            TokenType::Keyword(Keyword::Or) => Ok(LogicalOp::Or),
            _ => Err("Not a logical operator"),
        }
    }
}

// operator → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" | ;
#[derive(Debug, Clone, Display)]
pub enum BinaryOp {
    #[display("{}", TokenType::Equal.lexeme())]
    Equal,
    #[display("{}", TokenType::BangEqual.lexeme())]
    NotEqual,
    #[display("{}", TokenType::Less.lexeme())]
    Less,
    #[display("{}", TokenType::LessEqual.lexeme())]
    LessEqual,
    #[display("{}", TokenType::Greater.lexeme())]
    Greater,
    #[display("{}", TokenType::GreaterEqual.lexeme())]
    GreaterEqual,
    #[display("{}", TokenType::Plus.lexeme())]
    Plus,
    #[display("{}", TokenType::Minus.lexeme())]
    Minus,
    #[display("{}", TokenType::Star.lexeme())]
    Multiply,
    #[display("{}", TokenType::Slash.lexeme())]
    Divide,
}

impl TryFrom<&TokenType> for BinaryOp {
    type Error = &'static str;

    fn try_from(token_type: &TokenType) -> Result<Self, Self::Error> {
        match token_type {
            TokenType::Equal => Ok(BinaryOp::Equal),
            TokenType::BangEqual => Ok(BinaryOp::NotEqual),
            TokenType::Less => Ok(BinaryOp::Less),
            TokenType::LessEqual => Ok(BinaryOp::LessEqual),
            TokenType::Greater => Ok(BinaryOp::Greater),
            TokenType::GreaterEqual => Ok(BinaryOp::GreaterEqual),
            TokenType::Plus => Ok(BinaryOp::Plus),
            TokenType::Minus => Ok(BinaryOp::Minus),
            TokenType::Star => Ok(BinaryOp::Multiply),
            TokenType::Slash => Ok(BinaryOp::Divide),
            _ => Err("Not a binary operator"),
        }
    }
}

#[derive(Debug, Clone, Display)]
pub enum UnaryOp {
    #[display("{}", TokenType::Minus.lexeme())]
    Minus,
    #[display("{}", TokenType::Bang.lexeme())]
    Not,
}

impl TryFrom<&TokenType> for UnaryOp {
    type Error = &'static str;

    fn try_from(token_type: &TokenType) -> Result<Self, Self::Error> {
        match token_type {
            TokenType::Minus => Ok(UnaryOp::Minus),
            TokenType::Bang => Ok(UnaryOp::Not),
            _ => Err("Not a unary operator"),
        }
    }
}
