pub mod app;
mod circles_app;

#[macro_use]
extern crate slog;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

fn main() {
    let logger = init_logger();
    info!(logger, "Logger initialized");
}

fn init_logger() -> Logger {
    let term = TermDecorator::new().build();
    let format = CompactFormat::new(term).build().fuse();
    let sync = Async::new(format).build().fuse();
    Logger::root(sync, o!())
}
