//! What gets logged to Rerun, and under which entity paths.
//!
//! `world/...` is the absolute 2D scene; `relative/...` is the same data minus
//! ground truth, so the view zooms onto the error cloud around (0, 0).

use std::collections::VecDeque;

use fusion_rust::kf::types::{StateCovariance, StateVector, STATE_X, STATE_Y};
use fusion_rust::pipeline::Estimate;
use fusion_rust::replay::{SensorSample, TruthSample};
use rerun::components::Radius;
use rerun::{
    Color, LineStrips2D, Points2D, RecordingStream, RecordingStreamResult, Scalars, SeriesLines,
};

use crate::ellipse::position_ellipse;

const TIMELINE: &str = "sim_time";
/// 95 % bound of a chi-square with 2 degrees of freedom.
const NIS_BOUND_95: f64 = 5.991;
const SIGMA_SCALE: f64 = 2.0;
/// Samples kept in the trails (1 s at the baseline's 100 Hz).
const TRAIL_SAMPLES: usize = 100;
/// Half width of the faint square that fixes the relative view's extent [m].
const FRAME_HALF_WIDTH_M: f32 = 1.6;

// Sizes in UI points, so they do not grow when the view zooms in.
const POINT_UI: f32 = 3.5;
const MEASUREMENT_UI: f32 = 2.0;
const LINE_UI: f32 = 0.75;

const TRUTH_COLOR: [u8; 3] = [150, 150, 150];
const MEASUREMENT_COLOR: [u8; 3] = [230, 140, 40];
const FRAME_COLOR: [u8; 3] = [60, 60, 60];

/// Data shared by all runs that does not change over time.
pub fn static_context(
    rec: &RecordingStream,
    truth: &[TruthSample],
    sensor_std_m: f64,
) -> RecordingStreamResult<()> {
    let path: Vec<[f32; 2]> = truth.iter().map(|sample| position(&sample.state)).collect();
    rec.log_static("world/truth/path", &line(path, TRUTH_COLOR))?;
    rec.log_static(
        "relative/truth",
        &Points2D::new([[0.0, 0.0]])
            .with_colors([rgb(TRUTH_COLOR)])
            .with_radii([Radius::new_ui_points(POINT_UI)]),
    )?;
    let h = FRAME_HALF_WIDTH_M;
    let square = vec![[-h, -h], [h, -h], [h, h], [-h, h], [-h, -h]];
    rec.log_static("relative/frame", &line(square, FRAME_COLOR))?;
    let variance = sensor_std_m * sensor_std_m;
    let sensor_covariance =
        StateCovariance::from_diagonal(&StateVector::new(variance, variance, 0.0, 0.0));
    let sensor_circle = position_ellipse(&StateVector::zeros(), &sensor_covariance, SIGMA_SCALE);
    rec.log_static(
        "relative/sensor_2sigma",
        &line(sensor_circle, [110, 70, 30]),
    )?;
    rec.log_static(
        "plots/nis/bound_95",
        &SeriesLines::new()
            .with_colors([Color::from_rgb(200, 60, 60)])
            .with_names(["χ² 95 % (2 dof)"]),
    )
}

/// Logs one filter run. `label` (e.g. `rust_q0.01`) keeps runs apart in
/// entity paths; `name` (e.g. `q=0.01`) is what legends show. One colour per
/// run, derived from q, ties its 2D items and plot lines together.
pub struct RunLogger<'a> {
    rec: &'a RecordingStream,
    label: String,
    name: String,
    color: [u8; 3],
    measurement_trail: VecDeque<[f32; 2]>,
    estimate_trail: VecDeque<[f32; 2]>,
}

impl<'a> RunLogger<'a> {
    pub fn new(
        rec: &'a RecordingStream,
        acceleration_noise_density: f64,
    ) -> RecordingStreamResult<Self> {
        let logger = Self {
            rec,
            label: format!("rust_q{acceleration_noise_density}"),
            name: format!("q={acceleration_noise_density}"),
            color: run_color(acceleration_noise_density),
            measurement_trail: VecDeque::with_capacity(TRAIL_SAMPLES),
            estimate_trail: VecDeque::with_capacity(TRAIL_SAMPLES),
        };
        logger.name_series()?;
        Ok(logger)
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    /// One filter step at `estimate.timestamp`.
    pub fn step(
        &mut self,
        sample: &SensorSample,
        estimate: &Estimate,
        truth: Option<&TruthSample>,
    ) -> RecordingStreamResult<()> {
        self.rec.set_duration_secs(TIMELINE, estimate.timestamp.0);
        self.log_world(sample, estimate, truth)?;
        self.log_plots(estimate, truth)?;
        if let Some(truth) = truth {
            self.log_relative(sample, estimate, truth)?;
        }
        Ok(())
    }

    /// Legend names and colours; y is a darker shade of the run colour.
    fn name_series(&self) -> RecordingStreamResult<()> {
        let (label, name) = (&self.label, &self.name);
        let shades = [("x", self.color), ("y", self.color.map(|c| c / 2))];
        for kind in ["error", "sigma"] {
            for (axis, color) in shades {
                self.rec.log_static(
                    format!("plots/{kind}/{label}/{axis}"),
                    &SeriesLines::new()
                        .with_names([format!("{name} {axis}")])
                        .with_colors([rgb(color)]),
                )?;
            }
        }
        self.rec.log_static(
            format!("plots/nis/{label}"),
            &SeriesLines::new()
                .with_names([name.as_str()])
                .with_colors([rgb(self.color)]),
        )
    }

    fn log_world(
        &self,
        sample: &SensorSample,
        estimate: &Estimate,
        truth: Option<&TruthSample>,
    ) -> RecordingStreamResult<()> {
        let z = sample.measurement;
        self.rec.log(
            "world/measurements",
            &point([z.x as f32, z.y as f32], MEASUREMENT_COLOR, POINT_UI),
        )?;
        self.rec.log(
            format!("world/estimate/{}", self.label),
            &point(position(&estimate.state), self.color, POINT_UI),
        )?;
        if let Some(truth) = truth {
            self.rec.log(
                "world/truth/current",
                &point(position(&truth.state), TRUTH_COLOR, POINT_UI),
            )?;
        }
        Ok(())
    }

    /// Measurement cloud, estimate trail and 2σ ellipse, all minus truth.
    fn log_relative(
        &mut self,
        sample: &SensorSample,
        estimate: &Estimate,
        truth: &TruthSample,
    ) -> RecordingStreamResult<()> {
        let z = sample.measurement;
        let z_error = [
            (z.x - truth.state[STATE_X]) as f32,
            (z.y - truth.state[STATE_Y]) as f32,
        ];
        let estimate_error = estimate.state - truth.state;
        push_bounded(&mut self.measurement_trail, z_error);
        push_bounded(&mut self.estimate_trail, position(&estimate_error));

        let label = &self.label;
        self.rec.log(
            "relative/measurements",
            &Points2D::new(self.measurement_trail.iter().copied())
                .with_colors([rgb(MEASUREMENT_COLOR)])
                .with_radii([Radius::new_ui_points(MEASUREMENT_UI)]),
        )?;
        self.rec.log(
            format!("relative/{label}/trail"),
            &line(self.estimate_trail.iter().copied().collect(), self.color),
        )?;
        self.rec.log(
            format!("relative/{label}/estimate"),
            &point(position(&estimate_error), self.color, POINT_UI),
        )?;
        self.rec.log(
            format!("relative/{label}/sigma2"),
            &line(
                position_ellipse(&estimate_error, &estimate.covariance, SIGMA_SCALE),
                self.color,
            ),
        )
    }

    fn log_plots(
        &self,
        estimate: &Estimate,
        truth: Option<&TruthSample>,
    ) -> RecordingStreamResult<()> {
        let label = &self.label;
        self.rec.log(
            format!("plots/nis/{label}"),
            &Scalars::new([estimate.update.nis]),
        )?;
        self.rec
            .log("plots/nis/bound_95", &Scalars::new([NIS_BOUND_95]))?;
        for (axis, index) in [("x", STATE_X), ("y", STATE_Y)] {
            let sigma = estimate.covariance[(index, index)].max(0.0).sqrt();
            self.rec.log(
                format!("plots/sigma/{label}/{axis}"),
                &Scalars::new([sigma]),
            )?;
            if let Some(truth) = truth {
                let error = estimate.state[index] - truth.state[index];
                self.rec.log(
                    format!("plots/error/{label}/{axis}"),
                    &Scalars::new([error]),
                )?;
            }
        }
        Ok(())
    }
}

/// Blue for small q (smooth filter) through yellow for large q (responsive):
/// log10(q) from -3 to 1 maps onto the gradient.
fn run_color(acceleration_noise_density: f64) -> [u8; 3] {
    const SMOOTH: [f64; 3] = [70.0, 150.0, 255.0];
    const RESPONSIVE: [f64; 3] = [240.0, 200.0, 40.0];
    let t = ((acceleration_noise_density.log10() + 3.0) / 4.0).clamp(0.0, 1.0);
    [0, 1, 2].map(|i| (SMOOTH[i] + t * (RESPONSIVE[i] - SMOOTH[i])).round() as u8)
}

fn point(position: [f32; 2], color: [u8; 3], radius_ui: f32) -> Points2D {
    Points2D::new([position])
        .with_colors([rgb(color)])
        .with_radii([Radius::new_ui_points(radius_ui)])
}

fn line(points: Vec<[f32; 2]>, color: [u8; 3]) -> LineStrips2D {
    LineStrips2D::new([points])
        .with_colors([rgb(color)])
        .with_radii([Radius::new_ui_points(LINE_UI)])
}

fn push_bounded(trail: &mut VecDeque<[f32; 2]>, point: [f32; 2]) {
    if trail.len() == TRAIL_SAMPLES {
        trail.pop_front();
    }
    trail.push_back(point);
}

fn position(state: &StateVector) -> [f32; 2] {
    [state[STATE_X] as f32, state[STATE_Y] as f32]
}

fn rgb([r, g, b]: [u8; 3]) -> Color {
    Color::from_rgb(r, g, b)
}
