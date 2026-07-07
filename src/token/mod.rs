use std::fmt::Display;

pub use keyword::*;
use miette::SourceSpan;
pub use token_stream::*;
pub use token_type::*;

use crate::debug::{SourceMap, Debugable};

mod keyword;
mod token_stream;
mod token_type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    source_map: SourceMap,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl Token {
    pub fn new(token_type: TokenType, source_map: SourceMap) -> Self {
        Self { token_type, source_map }
    }
}

impl Debugable for Token {
    fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    fn line(&self) -> u32 {
        self.source_map.start_location.line
    }

    fn span(&self) -> SourceSpan {
        (&self.source_map).into()
    }
}
