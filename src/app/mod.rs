pub mod status;

use crate::app::status::Status;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;

pub trait App {
    type Event;

    fn process_event(&mut self, event: &Self::Event, wt: &EventLoopWindowTarget<()>) -> Status;
    fn update(&mut self, wt: &EventLoopWindowTarget<()>) -> Status;
    fn draw(&mut self, window_id: WindowId);
}
