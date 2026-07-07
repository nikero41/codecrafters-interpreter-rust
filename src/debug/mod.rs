mod location;
pub use location::*;

mod source_map;
pub use source_map::*;

pub trait Debugable {
    fn source_map(&self) -> &SourceMap;
    fn line(&self) -> u32;
    fn span(&self) -> miette::SourceSpan;
}
