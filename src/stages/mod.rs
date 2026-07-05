mod errors;
mod lexer;
mod parser;

pub use lexer::*;
pub use parser::*;

pub trait StageResult {
    fn print(&self);
    fn print_errors(&self);
    fn has_errors(&self) -> bool;
}
