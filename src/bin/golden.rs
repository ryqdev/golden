//! There are only two hard things in Computer Science:
//! 1. Cache invalidation
//! 2. Naming things

use std::io::Write;
use tokio::runtime::Builder;

// TODO: golden vs craft
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
/// Tokio is an asynchronous runtime for the Rust programming language.
/// It provides the building blocks needed for writing networking applications.
///
/// ## When not to use Tokio
/// 1. CPU-bound computations.
/// 2. Reading a lot of files.
/// 3. Sending a single web request.
///

fn main() {
    init_log();
    if let Err(err) = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Build tokio runtime failed")
        .block_on(cli::match_cmds())
    {
        println!("{:?}", err);
    }

}