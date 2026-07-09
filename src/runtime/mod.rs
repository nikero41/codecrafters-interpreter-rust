use std::rc::Rc;

use crate::{
    ast::declaration::Declaration,
    stages::{Parser, Scanner, StageResult},
    token::Token,
};

mod errors;
pub use errors::*;
use miette::SourceCode;

mod env;
pub use env::*;

#[derive(Debug)]
pub struct Interpreter<S>
where
    S: SourceCode + Clone + 'static,
{
    env: EnvironmentRef,
    source_debug: Option<S>,
}

impl<S: SourceCode + Clone + 'static> Interpreter<S> {
    pub fn new() -> Self {
        Self {
            env: Environment::new(None),
            source_debug: None,
        }
    }

    pub fn with_debug(&mut self, source_debug: S) -> &mut Self {
        self.source_debug = Some(source_debug);
        self
    }

    pub fn tokenize(&self, source_code: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source_code);
        scanner.scan();
        if scanner.has_errors() {
            if let Some(source_debug) = &self.source_debug {
                scanner.print_errors(source_debug.clone());
            } else {
                scanner.print_errors(source_code.to_string());
            }
            std::process::exit(65)
        }
        scanner.tokens()
    }

    pub fn parse(&self, source_code: &str) -> Vec<Declaration> {
        let tokens = self.tokenize(source_code);
        let mut parser = Parser::new(tokens);
        parser.parse();
        if parser.has_errors() {
            if let Some(source_debug) = &self.source_debug {
                parser.print_errors(source_debug.clone());
            } else {
                parser.print_errors(source_code.to_string());
            }
            std::process::exit(65)
        }
        parser.results()
    }

    pub fn run(&self, source_code: &str) {
        let statements = self.parse(source_code);

        statements.into_iter().for_each(|stmt| {
            if let Err(err) = stmt.execute(Rc::clone(&self.env)) {
                eprintln!("{}", err);
                std::process::exit(70);
            }
        });
    }

    pub fn print_ast(&self, source_code: &str) {
        let statements = self.parse(source_code);
        statements.into_iter().for_each(|stmt| println!("{}", stmt));
    }
}

impl<S: SourceCode + Clone + 'static> Default for Interpreter<S> {
    fn default() -> Self {
        Self::new()
    }
}
