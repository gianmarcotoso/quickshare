use tracing_subscriber::{EnvFilter, fmt::time::ChronoUtc};

const TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn init_logging(level: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_timer(ChronoUtc::new(TIMESTAMP_FORMAT.to_string()))
        .with_writer(std::io::stderr)
        .init();

    std::panic::set_hook(Box::new(|info| tracing::error!(panic = %info, "panic")));
}
