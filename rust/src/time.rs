//! Time spans in seconds, mirroring `fusion::Seconds` in C++.

/// Time span or timestamp in seconds. The unit lives in the type: a bare `f64`
/// is not accepted where `Seconds` is expected. `.0` yields the raw value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Seconds(pub f64);
