use std::fmt::Display;

pub use keyword::*;
use miette::SourceSpan;
pub use token_stream::*;
pub use token_type::*;

use crate::debug::DebugInfo;

mod keyword;
mod token_stream;
mod token_type;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub debug: DebugInfo,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl Token {
    pub fn new(token_type: TokenType, debug: DebugInfo) -> Self {
        Self { token_type, debug }
    }

    pub fn line(&self) -> u32 {
        self.debug.location.line
    }

    pub fn span(&self) -> SourceSpan {
        let span_length = self.token_type.lexeme().chars().count();
        if span_length == 1 {
            self.debug.location.offset.into()
        } else {
            (self.debug.location.offset, span_length).into()
        }
    }
}
