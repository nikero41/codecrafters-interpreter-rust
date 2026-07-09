use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum RuntimeError {
    #[diagnostic(code(runtime_error::not_a_number))]
    #[error("[line {line}] Operand must be a number.")]
    NotANumber {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[diagnostic(code(runtime_error::not_numbers))]
    #[error("[line {line}] Operands must be numbers.")]
    NotNumbers {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[diagnostic(code(runtime_error::invalid_operation))]
    #[error("[line {line}] Operands must be two numbers or two strings.")]
    InvalidOperation {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[diagnostic(code(runtime_error::undefined_variable))]
    #[error("[line {line}] Undefined variable '{name}'.")]
    UndefinedVariable {
        name: String,
        line: u32,
        #[label]
        span: SourceSpan,
    },
}
