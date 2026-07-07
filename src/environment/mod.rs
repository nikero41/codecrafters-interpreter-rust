use std::collections::HashMap;

use crate::{debug::Debugable, interpreter::RuntimeError, token::Token, values::LoxValue};

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    values: HashMap<String, LoxValue>,
    enclosing: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(parent: Option<&'a Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: parent,
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&LoxValue, RuntimeError> {
        match self.values.get(&name.token_type.lexeme()) {
            Some(value) => Ok(value),
            None => match self.enclosing {
                Some(env) => env.get(name),
                None => Err(RuntimeError::UndefinedVariable {
                    name: name.token_type.lexeme(),
                    line: name.line(),
                    span: name.span(),
                }),
            },
        }
    }
}

impl Default for Environment<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}
