mod errors;
mod lexer;
mod parser;

pub use errors::*;
pub use lexer::*;
pub use parser::*;

use miette::SourceCode;

pub trait StageResult {
    fn print(&self, source_code: impl SourceCode + Clone + 'static);
    fn print_errors(&self, source_code: impl SourceCode + Clone + 'static);
    fn has_errors(&self) -> bool;
}
