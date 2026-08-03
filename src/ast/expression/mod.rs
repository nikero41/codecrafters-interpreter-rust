use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    ast::{declaration::Declaration, statement::Stmt},
    debug::Debugable,
    runtime::{Environment, EnvironmentRef, RuntimeError},
    token::Token,
    values::LoxValue,
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
    /// unary → ( "-" | "!" ) expression | call ;
    Unary { operator: UnaryOp, right: Box<Expr> },
    /// call → primary ( "(" arguments? ")" )* ;
    /// arguments → expression ( "," expression )* ;
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    /// logic → logic ( "and" | "or" logic )* ;
    Logical {
        left: Box<Expr>,
        operator: LogicalOp,
        right: Box<Expr>,
    },
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
            Expr::Logical {
                left,
                operator,
                right,
            } => Self::eval_logical(*left, operator, *right, env),
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
            Expr::Call {
                callee, arguments, ..
            } => {
                let callee = callee.eval(Rc::clone(&env))?;

                let LoxValue::Callable {
                    arity, ref body, ..
                } = callee
                else {
                    let token = callee.token();
                    return Err(RuntimeError::NotCallable {
                        line: token.line(),
                        span: token.span(),
                    });
                };

                if arguments.len() != arity {
                    let token = callee.token();
                    return Err(RuntimeError::InvalidArguments {
                        line: token.line(),
                        expected: arity,
                        received: arguments.len(),
                        span: token.span(),
                    });
                }

                let func_env = Environment::new_sub(Rc::clone(&env));
                for arg in arguments {
                    let value = arg.eval(Rc::clone(&env))?;
                    func_env.borrow_mut().define("".to_string(), value);
                }

                for stmt in body.clone() {
                    stmt.execute(Rc::clone(&func_env))?
                }

                Ok(callee)
            }
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
                | LoxValue::Callable { .. }
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

    fn eval_logical(
        left: Expr,
        operator: LogicalOp,
        right: Expr,
        env: EnvironmentRef,
    ) -> Result<LoxValue, RuntimeError> {
        match operator {
            LogicalOp::And => {
                let left_value = left.eval(Rc::clone(&env))?;
                if !left_value.to_bool() {
                    Ok(left_value)
                } else {
                    right.eval(env)
                }
            }
            LogicalOp::Or => {
                let left_value = left.eval(Rc::clone(&env))?;
                if left_value.to_bool() {
                    Ok(left_value)
                } else {
                    right.eval(env)
                }
            }
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
                LoxValue::Callable { .. } => write!(f, "<native fn>"),
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
            Expr::Logical {
                left,
                right,
                operator,
            } => {
                let operator = match operator {
                    LogicalOp::And => "AND",
                    LogicalOp::Or => "OR",
                };
                write!(f, "({} {} {})", operator, left, right)
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                write!(
                    f,
                    "{}({})",
                    callee,
                    arguments
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
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
            Expr::Logical { left, .. } => left.source_map(),
            Expr::Call { callee, .. } => callee.source_map(),
        }
    }

    fn line(&self) -> u32 {
        self.source_map().line()
    }

    fn span(&self) -> miette::SourceSpan {
        match self {
            Expr::Literal { token, .. } => token.span(),
            Expr::Grouping(expr) => miette::SourceSpan::new(
                (expr.span().offset() - "(".len()).into(),
                expr.span().len() + "()".len(),
            ),
            Expr::Unary { right, .. } => (right.span().offset(), '-'.len_utf8()).into(),
            Expr::Binary {
                left,
                operator: _operator,
                right,
            } => {
                // TODO: take into account the length of the operator
                // let length = left.source_map().length + right.source_map().length;
                // (left.span().offset(), length).into()
                vec![left.span(), right.span()]
            }
            Expr::Assign { value, .. } => value.span(),
            Expr::Variable(token) => token.span(),
            Expr::Logical {
                left,
                right,
                operator,
            } => {
                let operator_length = match operator {
                    LogicalOp::And => "and".len(),
                    LogicalOp::Or => "or".len(),
                };

                let length = left.source_map().length + right.source_map().length + operator_length;
                (left.span().offset(), length).into()
            }
            Expr::Call { callee, .. } => callee.span(),
        }
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Stmt::Expr(expr)
    }
}

impl From<Expr> for Declaration {
    fn from(expr: Expr) -> Self {
        Declaration::Statement(Stmt::Expr(expr))
    }
}
