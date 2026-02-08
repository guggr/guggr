use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Setup logging
///
/// # Panics
/// Panics if `RUST_LOG` environment variable is unset and the default value
/// `info` cannot be set in a new [`EnvFilter`].
pub fn init_tracing() {
    let fmt_layer = fmt::layer().with_file(true).with_line_number(true).json(); // Keep JSON for production logs

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init(); // .init() is shorthand for set_global_default
}
