use std::io;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

/// Initialize optional file-based logging. Returns a guard that must be held alive.
pub fn init_logging(log_file: &str) -> io::Result<WorkerGuard> {
    let file = std::fs::File::create(log_file)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("stardial=debug"))
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    Ok(guard)
}
