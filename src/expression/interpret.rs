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
                let right = right.interpret()?;

                match operator {
                    UnaryOp::Minus => match right {
                        LoxValue::Number { value } => Ok(LoxValue::Number { value: -value }),
                        LoxValue::Object { .. }
                        | LoxValue::String { .. }
                        | LoxValue::Bool { .. }
                        | LoxValue::Nil { .. } => Err(InterpretError::InvalidOperator { line: 0 }),
                    },
                    UnaryOp::Not => Ok(LoxValue::Bool {
                        value: !right.to_bool(),
                    }),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.interpret()?;
                let right = right.interpret()?;

                match operator {
                    BinaryOp::Equal => left.eq(&right),
                    BinaryOp::NotEqual => {
                        if let LoxValue::Bool { value } = left.eq(&right)? {
                            Ok(LoxValue::Bool { value: !value })
                        } else {
                            panic!("Wut")
                        }
                    }
                    BinaryOp::Less => left.lt(&right),
                    BinaryOp::LessEqual => {
                        if let LoxValue::Bool { value: true } = left.lt(&right)? {
                            Ok(LoxValue::Bool { value: true })
                        } else if let LoxValue::Bool { value: true } = left.eq(&right)? {
                            Ok(LoxValue::Bool { value: true })
                        } else {
                            Ok(LoxValue::Bool { value: false })
                        }
                    }
                    BinaryOp::Greater => left.gt(&right),
                    BinaryOp::GreaterEqual => {
                        if let LoxValue::Bool { value: true } = left.gt(&right)? {
                            Ok(LoxValue::Bool { value: true })
                        } else if let LoxValue::Bool { value: true } = left.eq(&right)? {
                            Ok(LoxValue::Bool { value: true })
                        } else {
                            Ok(LoxValue::Bool { value: false })
                        }
                    }
                    BinaryOp::Plus => left.add(&right),
                    BinaryOp::Minus => left.subtract(&right),
                    BinaryOp::Multiply => left.multiply(&right),
                    BinaryOp::Divide => left.divide(&right),
                }
            }
        }
    }
}
