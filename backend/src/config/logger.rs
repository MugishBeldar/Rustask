use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Initialize the logger with custom formatting
/// Format: [TIMESTAMP] [LEVEL] [FILE:LINE] | message
pub fn init_logger() {
    // Get log level from environment variable or default to "info"
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Create a custom formatter
    let fmt_layer = fmt::layer()
        .with_target(true)           // Include the target (module path)
        .with_file(true)             // Include file name
        .with_line_number(true)      // Include line number
        .with_thread_ids(false)      // Don't include thread IDs
        .with_thread_names(false)    // Don't include thread names
        .with_span_events(FmtSpan::CLOSE)  // Log when spans close
        .compact();                  // Use compact format

    // Initialize the subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Logger initialized successfully");
}

