use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
#[error("[line {line}] Error: Invalid number: {number}")]
pub enum ScanError {
    #[error("[line {line}] Error: Invalid number: {number}")]
    InvalidNumber { line: u32, number: String },
    #[error("[line {line}] Error: Unexpected character: {character}")]
    InvalidCharacter { line: u32, character: char },
    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString {
        line: u32,
        #[label]
        span: SourceSpan,
        string: String,
    },
}
