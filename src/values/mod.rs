use std::fmt::{Display, Formatter};

use crate::{
    expression::{InterpretError, interpret::Interpretable},
    token::{Keyword, Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum LoxValue {
    Object {},
    Number { value: f64 },
    String { value: String },
    Bool { value: bool },
    Nil {},
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Number { value, .. } => write!(f, "{}", value),
            LoxValue::String { value, .. } => write!(f, "\"{}\"", value),
            LoxValue::Bool { value, .. } => write!(f, "{}", value),
            LoxValue::Nil { .. } => write!(f, "nil"),
            LoxValue::Object { .. } => write!(f, "object"),
        }
    }
}

impl TryFrom<&Token> for LoxValue {
    type Error = &'static str;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match &token.token_type {
            TokenType::Number(float) => Ok(LoxValue::Number {
                value: float.parse::<f64>().unwrap(),
            }),
            TokenType::String(literal) => Ok(LoxValue::String {
                value: literal.clone(),
            }),
            TokenType::Keyword(Keyword::True) => Ok(LoxValue::Bool { value: true }),
            TokenType::Keyword(Keyword::False) => Ok(LoxValue::Bool { value: false }),
            TokenType::Keyword(Keyword::Nil) => Ok(LoxValue::Nil {}),
            _ => Err("Not a literal type"),
        }
    }
}

impl LoxValue {
    pub fn to_bool(&self) -> bool {
        match self {
            LoxValue::Bool { value, .. } => *value,
            LoxValue::Number { value, .. } => *value > 0_f64,
            LoxValue::String { value, .. } => !value.is_empty(),
            LoxValue::Nil { .. } => false,
            LoxValue::Object { .. } => true,
        }
    }
}

impl Interpretable for LoxValue {
    fn interpret(&self) -> Result<LoxValue, InterpretError> {
        Ok(self.clone())
    }
}
