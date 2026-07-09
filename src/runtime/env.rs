use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{debug::Debugable, runtime::RuntimeError, token::Token, values::LoxValue};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<EnvironmentRef>,
}

pub type EnvironmentRef = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new(parent: Option<EnvironmentRef>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: parent,
        }))
    }

    pub fn new_sub(parent: EnvironmentRef) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: Some(parent),
        }))
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn mutate(&mut self, name_token: Token, value: LoxValue) -> Result<(), RuntimeError> {
        let name = name_token.token_type.lexeme();
        match self.values.get(&name) {
            Some(_) => {
                self.values.insert(name, value);
            }
            None => match &mut self.enclosing {
                Some(env) => env.borrow_mut().mutate(name_token, value),
                None => Err(RuntimeError::UndefinedVariable {
                    name,
                    line: name_token.line(),
                    span: name_token.span(),
                }),
            }?,
        }

        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, RuntimeError> {
        match self.values.get(&name.token_type.lexeme()) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(env) => env.borrow().get(name),
                None => Err(RuntimeError::UndefinedVariable {
                    name: name.token_type.lexeme(),
                    line: name.line(),
                    span: name.span(),
                }),
            },
        }
    }
}
