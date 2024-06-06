use std::io::Write;
use golden::cli::match_cmds;

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
/// In low trading system, assign one strategy with single thread runtime is possibly better.
///
/// TODO: need benchmark test
///
/// ```rust
/// fn main() {
///     init_log();
///     tokio::runtime::Builder::new_current_thread() // single thread mode
///        .enable_all() //Enables both I/ O and time drivers.
///         .build()
///         .expect("Build tokio runtime failed")
///         .block_on(cli::match_cmds()) // block_on will block main thread
///         .unwrap()
/// }
/// ```

// another simpler way to use tokio
// TODO: use `serveral` single-thread runtimes or `one` multi-thread runtime?
#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_log();
    match_cmds().await.expect("match error");
}

// https://tokio.rs/blog/2019-10-scheduler