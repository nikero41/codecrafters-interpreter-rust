use std::{fs, path::PathBuf, sync::Arc};

use miette::{Context, IntoDiagnostic, NamedSource, Result};

#[derive(Debug, PartialEq)]
pub struct SourceFile {
    path: PathBuf,
    pub content: String,
    pub named_source: NamedSource<Arc<String>>,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file_content = fs::read_to_string(&path)
            .into_diagnostic()
            .wrap_err(format!("Failed to read file {}", path.display()))?;

        let filename = path.display().to_string();

        Ok(Self {
            path,
            content: file_content.clone(),
            named_source: NamedSource::new(filename, Arc::new(file_content)),
        })
    }
}

impl From<&SourceFile> for NamedSource<Arc<String>> {
    fn from(value: &SourceFile) -> Self {
        NamedSource::new(
            value.path.to_string_lossy(),
            Arc::new(value.content.clone()),
        )
    }
}
