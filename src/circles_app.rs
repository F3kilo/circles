use crate::app::App;
use crate::app::status::Status;

struct CirclesApp;

impl App for CirclesApp {
    type Event = ();

    fn process_event(&mut self, _event: &Self::Event) -> Status {
        unimplemented!()
    }

    fn update(&mut self) -> Status {
        unimplemented!()
    }
}