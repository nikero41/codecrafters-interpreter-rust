use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum RuntimeError {
    #[error("[line {line}] Operand must be a number.")]
    NotANumber {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Operands must be numbers.")]
    NotNumbers {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Operands must be two numbers or two strings.")]
    InvalidOperation {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Operands must be two numbers or two strings.")]
    UndefinedVariable {
        name: String,
        line: u32,
        #[label]
        span: SourceSpan,
    },
}
