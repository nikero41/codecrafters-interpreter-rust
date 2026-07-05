use miette::SourceOffset;

use crate::source_file::SourceFile;

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    pub line: u32,
    pub column: u32,
}

impl DebugInfo {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn location(&self, source: &SourceFile) -> SourceOffset {
        let start_line = self.line as usize;
        let start_column = self.column as usize;
        SourceOffset::from_location(&source.content, start_line, start_column)
    }
}
