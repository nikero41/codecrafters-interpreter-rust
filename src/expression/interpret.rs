use crate::{
    expression::{BinaryOp, Expr, UnaryOp, errors::InterpretError},
    values::LoxValue,
};

pub trait Interpretable {
    fn interpret(&self) -> Result<LoxValue, InterpretError>;
}

impl Interpretable for Expr {
    fn interpret(&self) -> Result<LoxValue, InterpretError> {
        match self {
            Expr::Literal { value, .. } => value.interpret(),
            Expr::Grouping(expr) => expr.interpret(),
            Expr::Unary { operator, right } => {
                let lox_value = right.interpret()?;
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
                            Err(InterpretError::InvalidUnary {
                                line: token.line(),
                                debug: token.debug.clone(),
                            })
                        }
                    },
                    UnaryOp::Not => Ok(LoxValue::Bool {
                        value: !lox_value.to_bool(),
                        token: lox_value.token().clone(),
                    }),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = left.interpret()?;
                let right_value = right.interpret()?;

                match operator {
                    BinaryOp::Equal => left_value.eq(&right_value),
                    BinaryOp::NotEqual => {
                        if let LoxValue::Bool { value, token } = left_value.eq(&right_value)? {
                            Ok(LoxValue::Bool {
                                value: !value,
                                token,
                            })
                        } else {
                            panic!("Wut")
                        }
                    }
                    BinaryOp::Less => left_value.lt(&right_value),
                    BinaryOp::LessEqual => {
                        if let LoxValue::Bool { value: true, token } =
                            left_value.lt(&right_value)?
                        {
                            Ok(LoxValue::Bool { value: true, token })
                        } else if let LoxValue::Bool { value: true, token } =
                            left_value.eq(&right_value)?
                        {
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
                        if let LoxValue::Bool { value: true, token } =
                            left_value.gt(&right_value)?
                        {
                            Ok(LoxValue::Bool { value: true, token })
                        } else if let LoxValue::Bool { value: true, token } =
                            left_value.eq(&right_value)?
                        {
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
    }
}
