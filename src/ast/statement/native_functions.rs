use std::time::SystemTime;

use crate::{ast::statement::Stmt, token::Token, values::LoxValue};

#[derive(Debug, Clone)]
pub enum NativeFunction {
    Clock,
}

impl NativeFunction {
    pub fn arity(&self) -> usize {
        match self {
            NativeFunction::Clock => 0,
        }
    }

    pub fn call(&self, _args: Vec<LoxValue>) -> Result<LoxValue, String> {
        match self {
            NativeFunction::Clock => SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|duration| LoxValue::Number {
                    token: Token::new_dummy(),
                    value: duration.as_millis() as f64,
                })
                .map_err(|e| e.to_string()),
        }
    }
}

impl From<NativeFunction> for Stmt {
    fn from(native_function: NativeFunction) -> Self {
        Self::NativeFunction(native_function)
    }
}

impl From<NativeFunction> for LoxValue {
    fn from(native_function: NativeFunction) -> Self {
        Self::Callable {
            token: Token::new_dummy(),
            arity: 0,
            body: vec![Stmt::from(native_function)],
        }
    }
}
