use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum ScanError {
    #[error("[line {line}] Error: Unexpected character: {character}")]
    InvalidCharacter {
        line: u32,
        #[label("Unexpected character")]
        span: SourceSpan,
        character: char,
    },
    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString {
        line: u32,
        #[label]
        span: SourceSpan,
        string: String,
    },
}

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum ParseError {
    #[error("[line {line}] Error at end: {message}")]
    Eof { line: u32, message: String },
    #[error("[line {line}] Error at '{lexeme}': {message}")]
    ExpressionExpected {
        line: u32,
        lexeme: String,
        message: String,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Error: Unterminated paren.")]
    UnterminatedParen {
        line: u32,
        #[label]
        span: SourceSpan,
    },
}
