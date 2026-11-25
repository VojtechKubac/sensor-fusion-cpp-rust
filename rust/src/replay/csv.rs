//! Minimal reader for the replay CSV format: header row, then f64 fields.

use std::path::Path;

use super::ReplayError;

/// Reads `path`, checks the header equals `columns`, parses every data row
/// into `N` numbers. Runs once at startup, so returning a `Vec` is fine.
pub fn read_rows<const N: usize>(
    path: &Path,
    columns: [&str; N],
) -> Result<Vec<[f64; N]>, ReplayError> {
    // TODO(R2): read file, compare header, parse rows; report line numbers.
    let _ = (path, columns);
    todo!("read_rows")
}
