use lazy_static::lazy_static;
use slog;
use slog::{Logger, o, Drain};
use slog_term;
use slog_async;

lazy_static! {
    pub static ref LOGGER : Logger = configure_log();
}

pub(crate) fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();

    // It is used for Synchronization
    let console_drain = slog_async::Async::new(console_drain).build().fuse();

    // Root logger
    slog::Logger::root(console_drain, o!("v"=>env!("CARGO_PKG_VERSION")))
}