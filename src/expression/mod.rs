use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    debug::Debugable, environment::EnvironmentRef, interpreter::RuntimeError, statement::Stmt,
    token::Token, values::LoxValue,
};

mod operators;
pub use operators::*;

mod parser;
pub use parser::*;

#[derive(Debug, Clone)]
pub enum Expr {
    /// literal → NUMBER | STRING | "true" | "false" | "nil" ;
    Literal { value: LoxValue, token: Token },
    /// grouping → "(" expression ")" ;
    Grouping(Box<Expr>),
    /// unary → ( "-" | "!" ) expression ;
    Unary { operator: UnaryOp, right: Box<Expr> },
    /// binary → expression operator expression ;
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    /// assignment → IDENTIFIER "=" assignment ;
    Assign { token: Token, value: Box<Expr> },
    /// variable → IDENTIFIER ;
    Variable(Token),
}

impl Expr {
    pub fn eval(self, env: EnvironmentRef) -> Result<LoxValue, RuntimeError> {
        match self {
            Expr::Literal { value, .. } => Ok(value),
            Expr::Grouping(expr) => expr.eval(env),
            Expr::Unary { operator, right } => Self::eval_unary(operator, *right, env),
            Expr::Binary {
                left,
                operator,
                right,
            } => Self::eval_binary(*left, operator, *right, env),
            Expr::Assign { token, value } => {
                env.borrow().get(&token)?;
                let value = value.eval(Rc::clone(&env))?;
                env.borrow_mut().mutate(token, value.clone())?;
                Ok(value)
            }
            Expr::Variable(token) => env.borrow().get(&token),
        }
    }

    fn eval_unary(
        operator: UnaryOp,
        right: Expr,
        env: EnvironmentRef,
    ) -> Result<LoxValue, RuntimeError> {
        let lox_value = right.eval(env)?;
        match operator {
            UnaryOp::Minus => match lox_value {
                LoxValue::Number { value, token } => Ok(LoxValue::Number {
                    value: -value,
                    token,
                }),
                LoxValue::Object { .. }
                | LoxValue::String { .. }
                | LoxValue::Bool { .. }
                | LoxValue::Nil { .. } => {
                    let token = lox_value.token();
                    Err(RuntimeError::NotANumber {
                        line: token.line(),
                        span: token.span(),
                    })
                }
            },
            UnaryOp::Not => Ok(LoxValue::Bool {
                value: !lox_value.to_bool(),
                token: lox_value.token().clone(),
            }),
        }
    }

    fn eval_binary(
        left: Expr,
        operator: BinaryOp,
        right: Expr,
        env: EnvironmentRef,
    ) -> Result<LoxValue, RuntimeError> {
        let left_value = left.eval(Rc::clone(&env))?;
        let right_value = right.eval(env)?;

        match operator {
            BinaryOp::Equal => left_value.eq(&right_value),
            BinaryOp::NotEqual => {
                let value = left_value.eq(&right_value)?;
                Ok(LoxValue::Bool {
                    value: !value.to_bool(),
                    token: value.token().clone(),
                })
            }
            BinaryOp::Less => left_value.lt(&right_value),
            BinaryOp::LessEqual => {
                if let LoxValue::Bool { value: true, token } = left_value.lt(&right_value)? {
                    Ok(LoxValue::Bool { value: true, token })
                } else if let LoxValue::Bool { value: true, token } = left_value.eq(&right_value)? {
                    Ok(LoxValue::Bool { value: true, token })
                } else {
                    Ok(LoxValue::Bool {
                        value: false,
                        token: left_value.token().clone(),
                    })
                }
            }
            BinaryOp::Greater => left_value.gt(&right_value),
            BinaryOp::GreaterEqual => {
                if let LoxValue::Bool { value: true, token } = left_value.gt(&right_value)? {
                    Ok(LoxValue::Bool { value: true, token })
                } else if let LoxValue::Bool { value: true, token } = left_value.eq(&right_value)? {
                    Ok(LoxValue::Bool { value: true, token })
                } else {
                    Ok(LoxValue::Bool {
                        value: false,
                        token: left_value.token().clone(),
                    })
                }
            }
            BinaryOp::Plus => left_value.add(&right_value),
            BinaryOp::Minus => left_value.subtract(&right_value),
            BinaryOp::Multiply => left_value.multiply(&right_value),
            BinaryOp::Divide => left_value.divide(&right_value),
            BinaryOp::And => Ok(LoxValue::Bool {
                value: left_value.to_bool() && right_value.to_bool(),
                token: left_value.token().clone(),
            }),
            BinaryOp::Or => Ok(LoxValue::Bool {
                value: left_value.to_bool() || right_value.to_bool(),
                token: left_value.token().clone(),
            }),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal { value, .. } => match value {
                LoxValue::Object { .. } => write!(f, "TODO object"),
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
            Expr::Assign { token, value } => {
                write!(f, "{} = {}", token.token_type.lexeme(), value)
            }
            Expr::Variable(token) => {
                write!(f, "{}", token.token_type.lexeme())
            }
        }
    }
}

impl Debugable for Expr {
    fn source_map(&self) -> &crate::debug::SourceMap {
        match self {
            Expr::Literal { token, .. } => token.source_map(),
            Expr::Grouping(expr) => expr.source_map(),
            Expr::Unary { right, .. } => right.source_map(),
            Expr::Binary { left, .. } => left.source_map(),
            Expr::Assign { value, .. } => value.source_map(),
            Expr::Variable(token) => token.source_map(),
        }
    }

    fn line(&self) -> u32 {
        match self {
            Expr::Literal { token, .. } => token.line(),
            Expr::Grouping(expr) => expr.line(),
            Expr::Unary { right, .. } => right.line(),
            Expr::Binary { left, .. } => left.line(),
            Expr::Assign { value, .. } => value.line(),
            Expr::Variable(token) => token.line(),
        }
    }

    fn span(&self) -> miette::SourceSpan {
        match self {
            Expr::Literal { token, .. } => token.span(),
            Expr::Grouping(expr) => expr.span(),
            Expr::Unary { right, .. } => (right.span().offset(), '-'.len_utf8()).into(),
            Expr::Binary {
                left,
                operator: _operator,
                right,
            } => {
                // TODO: take into account the length of the operator
                let length = left.source_map().length + right.source_map().length;
                (left.span().offset(), length).into()
            }
            Expr::Assign { value, .. } => value.span(),
            Expr::Variable(token) => token.span(),
        }
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Stmt::Expr(expr)
    }
}
