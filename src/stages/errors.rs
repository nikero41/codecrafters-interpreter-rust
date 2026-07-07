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
    },
}

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum ParseError {
    #[error("[line {line}] Error at end: {message}")]
    Eof { line: u32, message: &'static str },
    #[error("[line {line}] Error at '{lexeme}': {message}")]
    ExpressionExpected {
        line: u32,
        lexeme: String,
        message: &'static str,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Expect {identifier_type}")]
    IdentifierExpected {
        line: u32,
        identifier_type: &'static str,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Error: Unterminated paren.")]
    UnterminatedParen {
        line: u32,
        #[label]
        span: SourceSpan,
    },
    #[error("[line {line}] Error: Invalid assignment target.")]
    InvalidAssignment {
        line: u32,
        #[label]
        span: SourceSpan,
    },
}

impl ParseError {
    pub fn needs_sync(&self) -> bool {
        !matches!(
            self,
            ParseError::InvalidAssignment { .. } | ParseError::Eof { .. }
        )
    }
}
