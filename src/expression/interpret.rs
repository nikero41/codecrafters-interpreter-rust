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
                        | LoxValue::Nil { .. } => {
                            Err(InterpretError::UnterminatedParen { line: 0 })
                        }
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
                    // BinaryOp::Equal => todo!(),
                    // BinaryOp::NotEqual => todo!(),
                    // BinaryOp::Less => todo!(),
                    // BinaryOp::LessEqual => todo!(),
                    // BinaryOp::Greater => todo!(),
                    // BinaryOp::GreaterEqual => todo!(),
                    BinaryOp::Plus => add(left, right),
                    // BinaryOp::Minus => left - right,
                    // BinaryOp::Multiply => left * right,
                    // BinaryOp::Divide => left / right,
                    _ => todo!(),
                }
            }
        }
    }
}

fn add(left: LoxValue, right: LoxValue) -> Result<LoxValue, InterpretError> {
    match (left, right) {
        (LoxValue::Number { value: a }, LoxValue::Number { value: b }) => {
            Ok(LoxValue::Number { value: a + b })
        }
        (LoxValue::String { .. }, LoxValue::String { .. }) => todo!(),

        (..) => todo!(),
    }
}
