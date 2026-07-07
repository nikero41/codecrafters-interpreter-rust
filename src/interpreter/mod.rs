use crate::environment::Environment;

mod errors;
pub use errors::*;

#[derive(Debug, Default)]
pub struct Interpreter<'a> {
    environment: Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }
}
