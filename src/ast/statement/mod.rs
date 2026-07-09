use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    ast::{declaration::Declaration, expression::Expr},
    runtime::{Environment, EnvironmentRef, RuntimeError},
};

mod parser;
pub use parser::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    /// printStmt → "print" expression ";" ;
    Print(Expr),
    /// exprStmt → expression ";" ;
    Expr(Expr),
    /// block → "{" declaration* "}" ;
    Block(Vec<Declaration>),
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
                let block_env = Environment::new_sub(Rc::clone(&env));
                stmts
                    .into_iter()
                    .try_for_each(|stmt| stmt.execute(Rc::clone(&block_env)))?
            }

            // Stmt::DeclareVar { name, expr } => {
            //     let value = if let Some(expr) = expr {
            //         expr.eval(Rc::clone(&env))?
            //     } else {
            //         LoxValue::Nil {
            //             token: name.clone(),
            //         }
            //     };
            //     env.borrow_mut()
            //         .define(name.token_type.lexeme(), value.clone());
            // }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let if_env = Environment::new_sub(Rc::clone(&env));
                let condition_value = condition.eval(Rc::clone(&if_env))?.to_bool();
                if condition_value {
                    then_branch.execute(Rc::clone(&if_env))?;
                } else if let Some(else_branch) = else_branch {
                    else_branch.execute(Rc::clone(&if_env))?;
                }
            }

            Stmt::While { condition, body } => {
                let while_env = Environment::new_sub(Rc::clone(&env));
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
            Stmt::Print(expr) => write!(f, "(Print {})", expr),
            Stmt::Expr(expr) => write!(f, "{}", expr),
            // Stmt::DeclareVar { name, expr } => {
            //     let expr_str = if let Some(expr) = expr {
            //         format!("{}", expr)
            //     } else {
            //         "nil".to_string()
            //     };
            //     write!(f, "(DeclareVar {} {})", name.token_type.lexeme(), expr_str)
            // }
            Stmt::Block(stmts) => {
                let stmts_str = stmts
                    .iter()
                    .map(|stmt| format!("{}", stmt))
                    .collect::<Vec<String>>()
                    .join("\n");

                write!(f, "(Block\n{}\n)", indent(stmts_str, 1))
            }
            Stmt::If {
                then_branch,
                else_branch,
                condition,
            } => {
                write!(f, "(If [{}]\n{}", condition, indent(then_branch, 1))?;
                if let Some(else_branch) = else_branch {
                    write!(f, "Else {}", indent(else_branch, 1))?;
                }
                write!(f, "\n)")?;
                Ok(())
            }
            Stmt::While { body, condition } => {
                write!(f, "(While [{}]\n{}\n)", condition, indent(body, 1))
            }
        }
    }
}

fn indent(value: impl Display, level: usize) -> String {
    let indent = "    ".repeat(level);
    value
        .to_string()
        .lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
}

impl From<Stmt> for Declaration {
    fn from(stmt: Stmt) -> Self {
        Declaration::Statement(stmt)
    }
}
