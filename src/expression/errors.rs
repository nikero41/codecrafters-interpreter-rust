use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum InterpretError {
    #[error("[line {line}] Error: Unterminated paren.")]
    UnterminatedParen {
        line: u32,
        // #[source_code]
        // src: NamedSource<Arc<String>>,
        // #[label]
        // span: SourceSpan,
    },
}
