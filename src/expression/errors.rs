use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum InterpretError {
    #[error("[line {line}] Operand must be a number.")]
    InvalidOperator {
        line: u32,
        // #[source_code]
        // src: NamedSource<Arc<String>>,
        // #[label]
        // span: SourceSpan,
    },
}
