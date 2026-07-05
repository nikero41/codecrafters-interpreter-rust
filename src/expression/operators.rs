use std::fmt::{Display, Formatter};

use crate::token::TokenType;

// operator → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Multiply,
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

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Equal => write!(f, "{}", TokenType::Equal.lexeme()),
            BinaryOp::NotEqual => write!(f, "{}", TokenType::BangEqual.lexeme()),
            BinaryOp::Less => write!(f, "{}", TokenType::Less.lexeme()),
            BinaryOp::LessEqual => write!(f, "{}", TokenType::LessEqual.lexeme()),
            BinaryOp::Greater => write!(f, "{}", TokenType::Greater.lexeme()),
            BinaryOp::GreaterEqual => write!(f, "{}", TokenType::GreaterEqual.lexeme()),
            BinaryOp::Plus => write!(f, "{}", TokenType::Plus.lexeme()),
            BinaryOp::Minus => write!(f, "{}", TokenType::Minus.lexeme()),
            BinaryOp::Multiply => write!(f, "{}", TokenType::Star.lexeme()),
            BinaryOp::Divide => write!(f, "{}", TokenType::Slash.lexeme()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
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

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Minus => write!(f, "{}", TokenType::Minus.lexeme()),
            UnaryOp::Not => write!(f, "{}", TokenType::Bang.lexeme()),
        }
    }
}
