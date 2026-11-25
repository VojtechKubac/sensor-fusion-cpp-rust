//! Minimal reader for the replay CSV format (sim/SCHEMA.md): exact header
//! row, then one sample per line of f64 fields, column 0 being a strictly
//! increasing timestamp. Mirrors `fusion/replay/csv_reader.hpp`.

use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsvErrorKind {
    /// First line missing or not the schema's header.
    HeaderMismatch,
    /// Row has more or fewer comma-separated fields.
    WrongFieldCount,
    /// Field is not a complete decimal number (or the line is unreadable).
    InvalidNumber,
    /// t <= previous row's t.
    NonIncreasingTimestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsvError {
    pub kind: CsvErrorKind,
    /// 1-based; the header is line 1.
    pub line: usize,
}

/// Parses `input` into rows of `N` numbers. Runs once before the replay loop,
/// so returning a `Vec` is fine.
pub fn read_rows<const N: usize>(
    input: impl BufRead,
    header: [&str; N],
) -> Result<Vec<[f64; N]>, CsvError> {
    let mut lines = input.lines();
    let header_ok = matches!(lines.next(), Some(Ok(line)) if line.split(',').eq(header));
    if !header_ok {
        return Err(CsvError {
            kind: CsvErrorKind::HeaderMismatch,
            line: 1,
        });
    }

    let mut rows: Vec<[f64; N]> = Vec::new();
    for (index, line) in lines.enumerate() {
        let line_number = index + 2;
        let error = |kind| CsvError {
            kind,
            line: line_number,
        };
        let text = line.map_err(|_| error(CsvErrorKind::InvalidNumber))?;
        let row: [f64; N] = parse_row(&text).map_err(error)?;
        if rows.last().is_some_and(|previous| row[0] <= previous[0]) {
            return Err(error(CsvErrorKind::NonIncreasingTimestamp));
        }
        rows.push(row);
    }
    Ok(rows)
}

fn parse_row<const N: usize>(text: &str) -> Result<[f64; N], CsvErrorKind> {
    let mut row = [0.0; N];
    let mut fields = text.split(',');
    for value in &mut row {
        let field = fields.next().ok_or(CsvErrorKind::WrongFieldCount)?;
        *value = field.parse().map_err(|_| CsvErrorKind::InvalidNumber)?;
    }
    if fields.next().is_some() {
        return Err(CsvErrorKind::WrongFieldCount);
    }
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: [&str; 3] = ["t", "zx", "zy"];

    fn read(text: &str) -> Result<Vec<[f64; 3]>, CsvError> {
        read_rows(text.as_bytes(), HEADER)
    }

    fn error(kind: CsvErrorKind, line: usize) -> Result<Vec<[f64; 3]>, CsvError> {
        Err(CsvError { kind, line })
    }

    #[test]
    fn reads_rows_after_the_header() {
        let rows = read("t,zx,zy\n0.0,1.5,-2\n0.01,3,4\n");

        assert_eq!(rows, Ok(vec![[0.0, 1.5, -2.0], [0.01, 3.0, 4.0]]));
    }

    #[test]
    fn rejects_a_missing_or_different_header() {
        assert_eq!(read(""), error(CsvErrorKind::HeaderMismatch, 1));
        assert_eq!(
            read("t,x,y\n0,1,2\n"),
            error(CsvErrorKind::HeaderMismatch, 1)
        );
    }

    #[test]
    fn reports_the_line_of_a_malformed_row() {
        let header_and_row = "t,zx,zy\n0,1,2\n";

        let too_few = format!("{header_and_row}1,2\n");
        let too_many = format!("{header_and_row}1,2,3,4\n");
        let not_a_number = format!("{header_and_row}1,2,abc\n");

        assert_eq!(read(&too_few), error(CsvErrorKind::WrongFieldCount, 3));
        assert_eq!(read(&too_many), error(CsvErrorKind::WrongFieldCount, 3));
        assert_eq!(read(&not_a_number), error(CsvErrorKind::InvalidNumber, 3));
    }

    #[test]
    fn rejects_a_timestamp_that_does_not_increase() {
        let rows = read("t,zx,zy\n0.5,1,2\n0.5,3,4\n");

        assert_eq!(rows, error(CsvErrorKind::NonIncreasingTimestamp, 3));
    }
}
