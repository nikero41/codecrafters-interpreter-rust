use std::fmt::{Display, Formatter};

use crate::{token::Token, values::LoxValue};

mod operators;
pub use operators::*;

mod errors;
pub use errors::*;

pub mod interpret;

/// expression     → literal | unary | binary | grouping ;
#[derive(Debug, Clone)]
pub enum Expr {
    /// literal → NUMBER | STRING | "true" | "false" | "nil" ;
    Literal { value: LoxValue, token: Token },
    /// grouping → "(" expression ")" ;
    Grouping(Box<Expr>),
    /// unary → ( "-" | "!" ) expression ;
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    /// binary → expression operator expression ;
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal { value, .. } => match value {
                LoxValue::Object {} => write!(f, "TODO object"),
                LoxValue::Number { value, .. } => write!(f, "{:?}", value),
                LoxValue::String { value, .. } => write!(f, "{}", value),
                LoxValue::Bool { value, .. } => {
                    if *value {
                        write!(f, "true")
                    } else {
                        write!(f, "false")
                    }
                }
                LoxValue::Nil { .. } => write!(f, "nil"),
            },
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator, right)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator, left, right)
            }
        }
    }
}
