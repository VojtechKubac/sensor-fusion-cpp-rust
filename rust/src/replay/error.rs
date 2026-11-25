use std::fmt;
use std::path::PathBuf;

use super::csv::CsvError;

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
    Csv {
        path: PathBuf,
        error: CsvError,
    },
    /// Files parse but do not match what this port expects.
    Unsupported(String),
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Json { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Csv { path, error } => {
                write!(f, "{}:{}: {:?}", path.display(), error.line, error.kind)
            }
            Self::Unsupported(reason) => write!(f, "unsupported replay: {reason}"),
        }
    }
}

impl std::error::Error for ReplayError {}
