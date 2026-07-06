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
            LoxValue::String { value, .. } => write!(f, "{}", value),
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

    pub fn add(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Number { value: a + b })
            }
            (LoxValue::String { value: a }, LoxValue::String { value: b }) => {
                Ok(LoxValue::String {
                    value: a.clone() + &b,
                })
            }

            (..) => todo!(),
        }
    }

    pub fn subtract(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Number { value: a - b })
            }
            (..) => todo!(),
        }
    }

    pub fn multiply(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Number { value: a * b })
            }
            (..) => todo!(),
        }
    }

    pub fn divide(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { .. }, LoxValue::Number { value: 0.0 }) => todo!(),
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Number { value: a / b })
            }
            (..) => todo!(),
        }
    }

    pub fn eq(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Bool { value: a == b })
            }
            (LoxValue::String { value: a }, LoxValue::String { value: b }) => {
                Ok(LoxValue::Bool { value: a == b })
            }
            (LoxValue::Bool { value: a }, LoxValue::Bool { value: b }) => {
                Ok(LoxValue::Bool { value: a == b })
            }
            (LoxValue::Nil {}, LoxValue::Nil {}) => Ok(LoxValue::Bool { value: true }),
            (_, LoxValue::Nil {}) | (LoxValue::Nil {}, _) => Ok(LoxValue::Bool { value: false }),

            (..) => todo!(),
        }
    }

    pub fn lt(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Bool { value: a < b })
            }
            (LoxValue::String { value: a }, LoxValue::String { value: b }) => {
                Ok(LoxValue::Bool { value: a < b })
            }
            (LoxValue::Bool { .. }, LoxValue::Bool { .. }) => Ok(LoxValue::Bool { value: false }),
            (LoxValue::Nil {}, LoxValue::Nil {})
            | (_, LoxValue::Nil {})
            | (LoxValue::Nil {}, _) => Ok(LoxValue::Bool { value: false }),

            (..) => todo!(),
        }
    }

    pub fn gt(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
                Ok(LoxValue::Bool { value: a > b })
            }
            (LoxValue::String { value: a }, LoxValue::String { value: b }) => {
                Ok(LoxValue::Bool { value: a > b })
            }
            (LoxValue::Bool { .. }, LoxValue::Bool { .. }) => Ok(LoxValue::Bool { value: false }),
            (LoxValue::Nil {}, LoxValue::Nil {})
            | (_, LoxValue::Nil {})
            | (LoxValue::Nil {}, _) => Ok(LoxValue::Bool { value: false }),

            (..) => todo!(),
        }
    }
}

impl Interpretable for LoxValue {
    fn interpret(&self) -> Result<LoxValue, InterpretError> {
        Ok(self.clone())
    }
}
