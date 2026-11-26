//! Viewer layout sent with every run, so live and saved recordings open the
//! same way. Each view covers a whole subtree (`<origin>/**`), so entities of
//! any run label (`rust_q0.01`, later `cpp_...`) appear without edits.

use rerun::blueprint::components::PanelState;
use rerun::blueprint::{
    Blueprint, BlueprintActivation, BlueprintPanel, Horizontal, SelectionPanel, Spatial2DView,
    TimePanel, TimeSeriesView, Vertical,
};
use rerun::{RecordingStream, RecordingStreamResult};

/// Scenes on the left (trajectory above the zoomed error view), plots on the
/// right; side panels collapsed so the views fill the window.
pub fn send(rec: &RecordingStream) -> RecordingStreamResult<()> {
    let scenes = Vertical::new([
        Spatial2DView::new("Trajectory: truth, measurements, estimates")
            .with_origin("world")
            .into(),
        Spatial2DView::new("Relative to truth: last 1 s, estimate ± 2σ, sensor 2σ circle [m]")
            .with_origin("relative")
            .into(),
    ])
    .with_row_shares([0.5, 0.5]);
    let plots = Vertical::new([
        TimeSeriesView::new("Error: estimate − truth [m]")
            .with_origin("plots/error")
            .into(),
        TimeSeriesView::new("σ: filter's own uncertainty [m]")
            .with_origin("plots/sigma")
            .into(),
        TimeSeriesView::new("NIS (should stay mostly under the 95 % bound)")
            .with_origin("plots/nis")
            .into(),
    ]);
    let layout = Horizontal::new([scenes.into(), plots.into()]).with_column_shares([0.45, 0.55]);
    Blueprint::new(layout)
        .with_blueprint_panel(BlueprintPanel::from_state(PanelState::Collapsed))
        .with_selection_panel(SelectionPanel::from_state(PanelState::Collapsed))
        .with_time_panel(TimePanel::new().with_state(PanelState::Collapsed))
        .send(rec, BlueprintActivation::default())
}
