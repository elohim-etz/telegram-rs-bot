use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

pub fn setup_logging(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .filter(None, level)
        .format_timestamp(None)
        .init();
}