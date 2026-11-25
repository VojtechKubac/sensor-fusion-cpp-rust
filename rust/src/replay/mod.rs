//! Reads the shared replay files described in sim/SCHEMA.md.

pub mod csv;
mod error;
mod meta;
mod samples;

pub use error::ReplayError;
pub use meta::{load_meta, InitialState, Meta};
pub use samples::{load_ground_truth, load_sensor, SensorSample, TruthSample};
