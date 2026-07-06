use std::fmt::Display;

pub use keyword::*;
use miette::{SourceOffset, SourceSpan};
pub use token_type::*;

use crate::{debug::DebugInfo, source_file::SourceFile};

mod keyword;
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
        self.debug.line
    }

    pub fn location(&self, source: &SourceFile) -> SourceOffset {
        self.debug.location(source)
    }

    pub fn span(&self, source: &SourceFile) -> SourceSpan {
        SourceSpan::new(self.location(source), self.token_type.lexeme().len())
    }
}
