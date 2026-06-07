use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("EOF reached")]
    EOF,
    #[error("[line {line}] Error: Invalid number: {number}")]
    InvalidNumber { line: u32, number: String },
    #[error("[line {line}] Error: Unexpected character: {character}")]
    InvalidCharacter { line: u32, character: char },
    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString { line: u32, string: String },
}
