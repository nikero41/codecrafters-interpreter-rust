use std::collections::HashMap;

use crate::{debug::Debugable, interpreter::RuntimeError, token::Token, values::LoxValue};

#[derive(Debug, Clone, Default)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&LoxValue, RuntimeError> {
        self.values
            .get(&name.token_type.lexeme())
            .ok_or(RuntimeError::UndefinedVariable {
                name: name.token_type.lexeme(),
                line: name.line(),
                span: name.span(),
            })
    }
}
