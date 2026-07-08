use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    environment::{Environment, EnvironmentRef},
    expression::Expr,
    interpreter::RuntimeError,
    token::Token,
    values::LoxValue,
};

mod parser;
pub use parser::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    /// varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
    DeclareVar { name: Token, expr: Option<Expr> },
    /// printStmt → "print" expression ";" ;
    Print(Expr),
    /// exprStmt → expression ";" ;
    Expr(Expr),
    /// block → "{" declaration* "}" ;
    Block(Vec<Stmt>),
    /// ifStmt → "if" "(" expression ")" statement ( "else" statement )? ;
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    /// whileStmt → "while" "(" expression ")" statement ;
    While { condition: Expr, body: Box<Stmt> },
}

impl Stmt {
    pub fn execute(self, env: EnvironmentRef) -> Result<(), RuntimeError> {
        match self {
            Stmt::Print(expr) => {
                let value = expr.eval(env)?;
                println!("{}", value);
            }
            Stmt::Expr(expr) => expr.eval(env).map(|_| ())?,

            Stmt::Block(stmts) => {
                let block_env = Rc::new(RefCell::new(Environment::new_sub(Rc::clone(&env))));
                stmts
                    .into_iter()
                    .try_for_each(|stmt| stmt.execute(Rc::clone(&block_env)))?
            }

            Stmt::DeclareVar { name, expr } => {
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

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let if_env = Rc::new(RefCell::new(Environment::new_sub(Rc::clone(&env))));
                let condition_value = condition.eval(Rc::clone(&if_env))?.to_bool();
                if condition_value {
                    then_branch.execute(Rc::clone(&if_env))?;
                } else if let Some(else_branch) = else_branch {
                    else_branch.execute(Rc::clone(&if_env))?;
                }
            }

            Stmt::While { condition, body } => {
                let while_env = Rc::new(RefCell::new(Environment::new_sub(Rc::clone(&env))));
                let mut condition_value = condition.clone().eval(Rc::clone(&while_env))?.to_bool();
                while condition_value {
                    body.clone().execute(Rc::clone(&while_env))?;
                    condition_value = condition.clone().eval(Rc::clone(&while_env))?.to_bool();
                }
            }
        }
        Ok(())
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Print(_) => write!(f, "PRINT"),
            Stmt::Expr(_) => write!(f, "EXPR"),
            Stmt::DeclareVar { .. } => write!(f, "DECLARE"),
            Stmt::Block { .. } => write!(f, "DECLARE"),
            Stmt::If { .. } => write!(f, "IF"),
            Stmt::While { .. } => write!(f, "WHILE"),
        }
    }
}
