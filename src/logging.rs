/// Enables logging through [pretty-env-logger].
///
/// A logger will **only** print errors from teloxide and **all** logs from
/// your program.
///
/// # Example
/// ```no_compile
/// teloxide::enable_logging!();
/// ```
///
/// # Note
/// Calling this macro **is not mandatory**; you can setup if your own logger if
/// you want.
///
/// [pretty-env-logger]: https://crates.io/crates/pretty_env_logger
#[macro_export]
macro_rules! enable_logging {
    () => {
        teloxide::enable_logging_with_filter!(log::LevelFilter::Trace);
    };
}

/// Enables logging through [pretty-env-logger] with a custom filter for your
/// program.
///
/// A logger will **only** print errors from teloxide and restrict logs from
/// your program by the specified filter.
///
/// # Example
/// Allow printing all logs from your program up to [`LevelFilter::Debug`] (i.e.
/// do not print traces):
///
/// ```no_compile
/// teloxide::enable_logging_with_filter!(log::LevelFilter::Debug);
/// ```
///
/// # Note
/// Calling this macro **is not mandatory**; you can setup if your own logger if
/// you want.
///
/// [pretty-env-logger]: https://crates.io/crates/pretty_env_logger
/// [`LevelFilter::Debug`]: https://docs.rs/log/0.4.10/log/enum.LevelFilter.html
#[macro_export]
macro_rules! enable_logging_with_filter {
    ($filter:expr) => {
        pretty_env_logger::formatted_builder()
            .write_style(pretty_env_logger::env_logger::WriteStyle::Auto)
            .filter(Some(&env!("CARGO_PKG_NAME").replace("-", "_")), $filter)
            .filter(Some("teloxide"), log::LevelFilter::Error)
            .init();
    };
}
