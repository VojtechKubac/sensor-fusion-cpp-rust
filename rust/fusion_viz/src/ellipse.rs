//! Confidence ellipse of a 2D Gaussian, as a closed polyline for Rerun.

use fusion_rust::kf::types::{StateCovariance, StateVector, STATE_X, STATE_Y};

const SEGMENTS: usize = 64;

/// Closed polyline of the `scale`-sigma ellipse of the position block of `p`
/// around the position in `x` (eigendecomposition of the 2x2 block).
#[must_use]
pub fn position_ellipse(x: &StateVector, p: &StateCovariance, scale: f64) -> Vec<[f32; 2]> {
    let block = p.fixed_view::<2, 2>(STATE_X, STATE_X).into_owned();
    let eigen = block.symmetric_eigen();
    let axes =
        [0, 1].map(|i| eigen.eigenvectors.column(i) * scale * eigen.eigenvalues[i].max(0.0).sqrt());
    let mut points: Vec<[f32; 2]> = (0..SEGMENTS)
        .map(|k| {
            let angle = std::f64::consts::TAU * k as f64 / SEGMENTS as f64;
            let offset = axes[0] * angle.cos() + axes[1] * angle.sin();
            [
                (x[STATE_X] + offset[0]) as f32,
                (x[STATE_Y] + offset[1]) as f32,
            ]
        })
        .collect();
    points.push(points[0]);
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_lie_on_the_scaled_sigma_ellipse_and_close_the_loop() {
        let x = StateVector::new(1.0, -2.0, 0.0, 0.0);
        let p = StateCovariance::from_diagonal(&StateVector::new(4.0, 1.0, 9.0, 9.0));

        let points = position_ellipse(&x, &p, 2.0);

        // 2-sigma semi-axes: 2 * sqrt(4) = 4 along x, 2 * sqrt(1) = 2 along y.
        for [px, py] in &points {
            let (dx, dy) = (f64::from(*px) - 1.0, f64::from(*py) + 2.0);
            let radius = (dx / 4.0).powi(2) + (dy / 2.0).powi(2);
            assert!((radius - 1.0).abs() < 1e-5, "({px}, {py}) -> {radius}");
        }
        assert_eq!(points.first(), points.last());
    }
}
