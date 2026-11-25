use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ReplayError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    /// Header mismatch, wrong column count or a non-numeric field.
    Csv {
        path: PathBuf,
        line: usize,
        reason: String,
    },
    /// Files parse but do not match what this port expects.
    Unsupported(String),
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Json { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Csv { path, line, reason } => write!(f, "{}:{line}: {reason}", path.display()),
            Self::Unsupported(reason) => write!(f, "unsupported replay: {reason}"),
        }
    }
}

impl std::error::Error for ReplayError {}
