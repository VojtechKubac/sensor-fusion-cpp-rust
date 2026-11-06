//! Placeholder until the C++ linear KF baseline is ported (feature loop 1).

fn main() {
    let mut args = std::env::args().skip(1);
    if matches!(args.next().as_deref(), Some("-h" | "--help")) {
        println!("fusion_rust — Rust sensor-fusion pipeline (stub)");
        println!("Usage: fusion_rust [--help]");
        return;
    }
    println!("fusion_rust stub — awaiting linear KF baseline");
}
