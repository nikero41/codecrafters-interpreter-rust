use std::fmt::{Display, Formatter};

use crate::{
    expression::{InterpretError, interpret::Interpretable},
    token::{Keyword, Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum LoxValue {
    Object { token: Token },
    Number { value: f64, token: Token },
    String { value: String, token: Token },
    Bool { value: bool, token: Token },
    Nil { token: Token },
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
                token: token.clone(),
            }),
            TokenType::String(literal) => Ok(LoxValue::String {
                value: literal.clone(),
                token: token.clone(),
            }),
            TokenType::Keyword(Keyword::True) => Ok(LoxValue::Bool {
                value: true,
                token: token.clone(),
            }),
            TokenType::Keyword(Keyword::False) => Ok(LoxValue::Bool {
                value: false,
                token: token.clone(),
            }),
            TokenType::Keyword(Keyword::Nil) => Ok(LoxValue::Nil {
                token: token.clone(),
            }),
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

    pub fn token(&self) -> &Token {
        match self {
            LoxValue::Object { token } => token,
            LoxValue::Number { token, .. } => token,
            LoxValue::String { token, .. } => token,
            LoxValue::Bool { token, .. } => token,
            LoxValue::Nil { token } => token,
        }
    }

    pub fn add(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a, .. }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Number {
                    value: a + b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::String { value: a, .. }, LoxValue::String { value: b, .. }) => {
                Ok(LoxValue::String {
                    value: a.clone() + b,
                    token: self.token().clone(),
                })
            }
            (..) => Err(InterpretError::InvalidAddition {
                line: self.token().line(),
                debug: self.token().debug.clone(),
            }),
        }
    }

    pub fn subtract(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a, token }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Number {
                    value: a - b,
                    token: token.clone(),
                })
            }
            (..) => Err(InterpretError::InvalidAddition {
                line: self.token().line(),
                debug: self.token().debug.clone(),
            }),
        }
    }

    pub fn multiply(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a, .. }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Number {
                    value: a * b,
                    token: self.token().clone(),
                })
            }
            (..) => Err(InterpretError::NotNumbers {
                line: self.token().line(),
                debug: self.token().debug.clone(),
            }),
        }
    }

    pub fn divide(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { .. }, LoxValue::Number { value: 0.0, .. }) => todo!(),
            (LoxValue::Number { value: a, token }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Number {
                    value: a / b,
                    token: token.clone(),
                })
            }
            (..) => Err(InterpretError::NotNumbers {
                line: self.token().line(),
                debug: self.token().debug.clone(),
            }),
        }
    }

    pub fn eq(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a, .. }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a == b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::String { value: a, .. }, LoxValue::String { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a == b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::Bool { value: a, .. }, LoxValue::Bool { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a == b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::Nil { .. }, LoxValue::Nil { .. }) => Ok(LoxValue::Bool {
                value: true,
                token: self.token().clone(),
            }),
            (_, LoxValue::Nil { .. }) | (LoxValue::Nil { .. }, _) => Ok(LoxValue::Bool {
                value: false,
                token: self.token().clone(),
            }),

            (..) => Ok(LoxValue::Bool {
                value: false,
                token: self.token().clone(),
            }),
        }
    }

    pub fn lt(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a, .. }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a < b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::String { value: a, .. }, LoxValue::String { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a < b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::Nil { .. }, LoxValue::Nil { .. })
            | (_, LoxValue::Nil { .. })
            | (LoxValue::Nil { .. }, _) => Ok(LoxValue::Bool {
                value: false,
                token: self.token().clone(),
            }),

            (..) => Err(InterpretError::NotNumbers {
                line: self.token().line(),
                debug: self.token().debug.clone(),
            }),
        }
    }

    pub fn gt(&self, right: &LoxValue) -> Result<LoxValue, InterpretError> {
        match (self, right) {
            (LoxValue::Number { value: a, .. }, LoxValue::Number { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a > b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::String { value: a, .. }, LoxValue::String { value: b, .. }) => {
                Ok(LoxValue::Bool {
                    value: a > b,
                    token: self.token().clone(),
                })
            }
            (LoxValue::Nil { .. }, LoxValue::Nil { .. })
            | (_, LoxValue::Nil { .. })
            | (LoxValue::Nil { .. }, _) => Ok(LoxValue::Bool {
                value: false,
                token: self.token().clone(),
            }),

            (..) => Err(InterpretError::NotNumbers {
                line: self.token().line(),
                debug: self.token().debug.clone(),
            }),
        }
    }
}

impl Interpretable for LoxValue {
    fn interpret(&self) -> Result<LoxValue, InterpretError> {
        Ok(self.clone())
    }
}
