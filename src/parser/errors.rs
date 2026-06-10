use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
    #[error("Placeholder")]
    Placeholder,
    #[error("[line {line}] Error: Unterminated paren.")]
    UnterminatedParen { line: u32 },
}
