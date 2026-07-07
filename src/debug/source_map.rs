use miette::SourceSpan;

use crate::debug::{Debugable, Location};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceMap {
    pub start_location: Location,
    pub length: usize,
}

impl SourceMap {
    pub fn new(start_location: Location, length: usize) -> Self {
        Self {
            start_location,
            length,
        }
    }
}

impl From<Location> for SourceMap {
    fn from(location: Location) -> Self {
        Self {
            start_location: location,
            length: 1,
        }
    }
}

impl Debugable for SourceMap {
    fn source_map(&self) -> &SourceMap {
        self
    }

    fn line(&self) -> u32 {
        self.start_location.line
    }

    fn span(&self) -> SourceSpan {
        SourceSpan::from(self)
    }
}

impl From<&SourceMap> for SourceSpan {
    fn from(value: &SourceMap) -> Self {
        if value.length == 1 {
            value.start_location.offset.into()
        } else {
            (value.start_location.offset, value.length).into()
        }
    }
}
