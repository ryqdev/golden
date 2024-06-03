use std::io::Write;
use golden::cli;

pub fn init_log() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown_file"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();
}

/// https://tokio.rs/tokio/tutorial
/// # Tokio
/// Tokio is an asynchronous runtime for the Rust programming language, especially for non-blocking IO
///
/// ## When not to use Tokio
/// 1. CPU-bound computations.
/// 2. Reading a lot of files.
/// 3. Sending a single web request.
///
/// ## Tokio provides two runtime modes:
/// 1. single thread runtime: new_current_thread
/// 2. multi thread runtime: new_multi_thread
///
/// In low latency trading system, possibly single thread is better.
///
fn main() {
    init_log();
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("Build tokio runtime failed")
        .block_on(cli::match_cmds())
        .unwrap()
}