//! Replays a simulator run through the linear KF and prints sanity metrics.

use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use fusion_rust::kf::types::{STATE_X, STATE_Y};
use fusion_rust::pipeline::{run_filter, truth_at, FilterTuning};
use fusion_rust::replay::{load_ground_truth, load_meta, load_sensor};

const DEFAULT_REPLAY_DIR: &str = "sim/out/baseline";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let replay_dir = match args.next() {
        Some(flag) if flag == "-h" || flag == "--help" => {
            println!("Usage: fusion_rust [REPLAY_DIR]   (default: {DEFAULT_REPLAY_DIR})");
            return ExitCode::SUCCESS;
        }
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(DEFAULT_REPLAY_DIR),
    };
    match run(&replay_dir) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("fusion_rust: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(replay_dir: &std::path::Path) -> Result<(), Box<dyn Error>> {
    let meta = load_meta(replay_dir)?;
    let sensor = load_sensor(replay_dir)?;
    let truth = load_ground_truth(replay_dir)?;

    let mut squared_error_sum = 0.0;
    let mut nis_sum = 0.0;
    let mut matched = 0_u32;
    run_filter(
        &sensor,
        meta.noise_std_m,
        &FilterTuning::default(),
        |_, estimate| {
            nis_sum += estimate.update.nis;
            if let Some(reference) = truth_at(&truth, estimate.timestamp) {
                let error = estimate.state - reference.state;
                squared_error_sum += error[STATE_X].powi(2) + error[STATE_Y].powi(2);
                matched += 1;
            }
        },
    )
    .map_err(|error| format!("filter update failed: {error:?}"))?;

    let count = f64::from(matched.max(1));
    println!("samples:        {}", sensor.len());
    println!("matched truth:  {matched}");
    println!(
        "position RMSE:  {:.4} m",
        (squared_error_sum / count).sqrt()
    );
    println!("sensor sigma:   {:.4} m per axis", meta.noise_std_m);
    println!(
        "mean NIS:       {:.3} (expect ~2 for 2 dof)",
        nis_sum / sensor.len().max(1) as f64
    );
    Ok(())
}
