pub mod status;

use crate::app::status::Status;

pub trait App {
    type Event;

    fn process_event(&mut self, event: &Self::Event) -> Status;
    fn update(&mut self) -> Status;
}
