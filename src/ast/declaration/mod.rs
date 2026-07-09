use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    ast::{expression::Expr, statement::Stmt},
    runtime::{EnvironmentRef, RuntimeError},
    token::Token,
    values::LoxValue,
};

mod parser;
pub use parser::*;

#[derive(Debug, Clone)]
pub enum Declaration {
    /// varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
    Var {
        name: Token,
        expr: Option<Expr>,
    },
    Statement(Stmt),
}

impl Declaration {
    pub fn execute(self, env: EnvironmentRef) -> Result<(), RuntimeError> {
        match self {
            Declaration::Var { name, expr } => {
                let value = if let Some(expr) = expr {
                    expr.eval(Rc::clone(&env))?
                } else {
                    LoxValue::Nil {
                        token: name.clone(),
                    }
                };
                env.borrow_mut()
                    .define(name.token_type.lexeme(), value.clone());
            }
            Declaration::Statement(stmt) => stmt.execute(Rc::clone(&env))?,
        }
        Ok(())
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Var { name, expr } => {
                let expr_str = if let Some(expr) = expr {
                    format!("{}", expr)
                } else {
                    "nil".to_string()
                };
                write!(f, "(DeclareVar {} {})", name.token_type.lexeme(), expr_str)
            }
            Declaration::Statement(stmt) => write!(f, "{}", stmt),
        }
    }
}
