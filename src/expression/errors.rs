use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum InterpretError {
    #[error("[line {line}] Operand must be a number.")]
    InvalidUnary {
        line: u32,
        // #[source_code]
        // src: NamedSource<Arc<String>>,
        // #[label]
        // span: SourceSpan,
    },
    #[error("[line {line}] Operands must be numbers.")]
    InvalidOperators {
        line: u32,
        // #[source_code]
        // src: NamedSource<Arc<String>>,
        // #[label]
        // span: SourceSpan,
    },
    #[error("[line {line}] Operands must be two numbers or two strings.")]
    InvalidAddition {
        line: u32,
        // #[source_code]
        // src: NamedSource<Arc<String>>,
        // #[label]
        // span: SourceSpan,
    },
}
