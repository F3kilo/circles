pub mod app;
mod circles_app;
pub mod vulkan;

#[macro_use]
extern crate slog;
use crate::app::App;
use crate::circles_app::CirclesApp;
use app::status::Status;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let logger = init_logger();
    info!(logger, "Logger initialized");

    let event_loop = EventLoop::new();
    trace!(logger, "Event loop initialized");

    let mut app = CirclesApp::new(logger.clone(), (800f32, 600f32).into(), &event_loop);
    info!(logger, "App initialized");

    event_loop.run(move |event, event_loop_wt, control_flow| {
        *control_flow = ControlFlow::Poll;
        if let Status::Finish = app.process_event(&(), event_loop_wt) {
            *control_flow = ControlFlow::Exit;
            trace!(logger, "Application finished. Exitting from event loop.");
            return;
        }
        match event {
            Event::MainEventsCleared => {
                trace!(logger, "Main events cleared. Updating presenter");
                if let Status::Finish = app.update(event_loop_wt) {
                    *control_flow = ControlFlow::Exit;
                    trace!(logger, "Application finished. Exitting from event loop.");
                    return;
                }
            }
            Event::RedrawRequested(window_id) => app.draw(window_id),
            _ => {}
        };
    });
}

fn init_logger() -> Logger {
    let term = TermDecorator::new().build();
    let format = CompactFormat::new(term).build().fuse();
    let sync = Async::new(format).build().fuse();
    Logger::root(sync, o!())
}
