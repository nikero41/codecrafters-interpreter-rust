use std::ops::Deref;

use derive_more::Display;
pub use keyword::*;
use miette::SourceSpan;
pub use token_stream::*;
pub use token_type::*;

use crate::debug::{Debugable, Location, SourceMap};

mod keyword;
mod token_stream;
mod token_type;

#[derive(Debug, Eq, PartialEq, Clone, Display)]
#[display("{}", token_type)]
pub struct Token {
    pub token_type: TokenType,
    source_map: SourceMap,
}

impl Token {
    pub fn new(token_type: TokenType, source_map: SourceMap) -> Self {
        Self {
            token_type,
            source_map,
        }
    }

    pub fn new_dummy() -> Self {
        Self {
            token_type: TokenType::Dummy,
            source_map: SourceMap::new(Location::default(), 1),
        }
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

impl Deref for Token {
    type Target = TokenType;

    fn deref(&self) -> &Self::Target {
        &self.token_type
    }
}
