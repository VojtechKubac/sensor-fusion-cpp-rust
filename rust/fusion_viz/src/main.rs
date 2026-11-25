//! Streams the linear-KF replay to a spawned Rerun viewer and an `.rrd` file.

mod ellipse;
mod log;

use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use fusion_rust::pipeline::{run_filter, truth_at, FilterTuning};
use fusion_rust::replay::{load_ground_truth, load_meta, load_sensor};
use fusion_rust::time::Seconds;
use rerun::sink::{FileSink, GrpcSink};

const USAGE: &str = "Usage: fusion_viz [--realtime] [--save FILE.rrd] [REPLAY_DIR]\n  \
    defaults: REPLAY_DIR=sim/out/baseline, FILE=report/out/loop1/loop1.rrd";

struct Options {
    replay_dir: PathBuf,
    rrd_path: PathBuf,
    /// Sleep so sim time advances like wall-clock time.
    realtime: bool,
}

fn main() -> ExitCode {
    let options = match parse_args(std::env::args().skip(1)) {
        Ok(Some(options)) => options,
        Ok(None) => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(message) => {
            eprintln!("fusion_viz: {message}\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };
    match run(&options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("fusion_viz: {error}");
            ExitCode::FAILURE
        }
    }
}

/// `Ok(None)` means `--help` was asked for.
fn parse_args(args: impl Iterator<Item = String>) -> Result<Option<Options>, String> {
    let mut options = Options {
        replay_dir: PathBuf::from("sim/out/baseline"),
        rrd_path: PathBuf::from("report/out/loop1/loop1.rrd"),
        realtime: false,
    };
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--realtime" => options.realtime = true,
            "--save" => options.rrd_path = args.next().ok_or("--save needs a path")?.into(),
            flag if flag.starts_with('-') => return Err(format!("unknown flag {flag}")),
            dir => options.replay_dir = dir.into(),
        }
    }
    Ok(Some(options))
}

fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    let meta = load_meta(&options.replay_dir)?;
    let sensor = load_sensor(&options.replay_dir)?;
    let truth = load_ground_truth(&options.replay_dir)?;

    let rec = connect(&options.rrd_path)?;
    log::static_context(&rec, &truth)?;

    let started = Instant::now();
    let mut first_log_error = None;
    run_filter(
        &sensor,
        meta.noise_std_m,
        &FilterTuning::default(),
        |sample, estimate| {
            if options.realtime {
                wait_until(started, estimate.timestamp);
            }
            let reference = truth_at(&truth, estimate.timestamp);
            if let Err(error) = log::step(&rec, sample, estimate, reference) {
                first_log_error.get_or_insert(error);
            }
        },
    )
    .map_err(|error| format!("filter update failed: {error:?}"))?;
    if let Some(error) = first_log_error {
        return Err(error.into());
    }

    rec.flush_blocking()?;
    println!("saved {}", options.rrd_path.display());
    Ok(())
}

/// Spawns (or reuses) a local viewer and tees the stream into `rrd_path`.
fn connect(rrd_path: &std::path::Path) -> Result<rerun::RecordingStream, Box<dyn Error>> {
    if let Some(parent) = rrd_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let rec = rerun::RecordingStreamBuilder::new("kalman_loop1").buffered()?;
    rerun::spawn(&rerun::SpawnOptions::default())?;
    rec.set_sinks((GrpcSink::default(), FileSink::new(rrd_path)?));
    Ok(rec)
}

fn wait_until(started: Instant, sim_time: Seconds) {
    let target = Duration::from_secs_f64(sim_time.0.max(0.0));
    std::thread::sleep(target.saturating_sub(started.elapsed()));
}
