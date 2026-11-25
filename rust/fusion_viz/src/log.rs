//! What gets logged to Rerun, and under which entity paths.

use fusion_rust::kf::types::{STATE_X, STATE_Y};
use fusion_rust::pipeline::Estimate;
use fusion_rust::replay::{SensorSample, TruthSample};
use rerun::{
    Color, LineStrips2D, Points2D, RecordingStream, RecordingStreamResult, Scalars, SeriesLines,
};

use crate::ellipse::position_ellipse;

const TIMELINE: &str = "sim_time";
/// Port name in entity paths, so a C++ run can be logged next to it later.
const PORT: &str = "rust";
/// 95 % bound of a chi-square with 2 degrees of freedom.
const NIS_BOUND_95: f64 = 5.991;
const SIGMA_SCALE: f64 = 2.0;

const TRUTH_COLOR: [u8; 3] = [120, 120, 120];
const MEASUREMENT_COLOR: [u8; 3] = [230, 140, 40];
const ESTIMATE_COLOR: [u8; 3] = [40, 120, 230];

/// Data that does not change over time: the full truth trajectory and plot styles.
pub fn static_context(rec: &RecordingStream, truth: &[TruthSample]) -> RecordingStreamResult<()> {
    let path: Vec<[f32; 2]> = truth.iter().map(|sample| position(sample.state)).collect();
    rec.log_static(
        "world/truth/path",
        &LineStrips2D::new([path]).with_colors([rgb(TRUTH_COLOR)]),
    )?;
    rec.log_static(
        "plots/nis/bound_95",
        &SeriesLines::new()
            .with_colors([Color::from_rgb(200, 60, 60)])
            .with_names(["χ² 95 % (2 dof)"]),
    )
}

/// One filter step at `estimate.timestamp`.
pub fn step(
    rec: &RecordingStream,
    sample: &SensorSample,
    estimate: &Estimate,
    truth: Option<&TruthSample>,
) -> RecordingStreamResult<()> {
    rec.set_duration_secs(TIMELINE, estimate.timestamp.0);
    let z = sample.measurement;
    rec.log(
        "world/measurements",
        &Points2D::new([[z.x as f32, z.y as f32]])
            .with_colors([rgb(MEASUREMENT_COLOR)])
            .with_radii([0.08]),
    )?;
    rec.log(
        format!("world/estimate/{PORT}"),
        &Points2D::new([position(estimate.state)])
            .with_colors([rgb(ESTIMATE_COLOR)])
            .with_radii([0.1]),
    )?;
    rec.log(
        format!("world/estimate/{PORT}/sigma2"),
        &LineStrips2D::new([position_ellipse(
            &estimate.state,
            &estimate.covariance,
            SIGMA_SCALE,
        )])
        .with_colors([rgb(ESTIMATE_COLOR)]),
    )?;
    rec.log(
        format!("plots/nis/{PORT}"),
        &Scalars::new([estimate.update.nis]),
    )?;
    rec.log("plots/nis/bound_95", &Scalars::new([NIS_BOUND_95]))?;
    for (axis, index) in [("x", STATE_X), ("y", STATE_Y)] {
        let sigma = estimate.covariance[(index, index)].max(0.0).sqrt();
        rec.log(format!("plots/sigma/{PORT}/{axis}"), &Scalars::new([sigma]))?;
    }
    if let Some(truth) = truth {
        rec.log(
            "world/truth/current",
            &Points2D::new([position(truth.state)])
                .with_colors([rgb(TRUTH_COLOR)])
                .with_radii([0.1]),
        )?;
        for (axis, index) in [("x", STATE_X), ("y", STATE_Y)] {
            let error = estimate.state[index] - truth.state[index];
            rec.log(format!("plots/error/{PORT}/{axis}"), &Scalars::new([error]))?;
        }
    }
    Ok(())
}

fn position(state: fusion_rust::kf::types::StateVector) -> [f32; 2] {
    [state[STATE_X] as f32, state[STATE_Y] as f32]
}

fn rgb([r, g, b]: [u8; 3]) -> Color {
    Color::from_rgb(r, g, b)
}
