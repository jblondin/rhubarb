use std::panic::RefUnwindSafe;

use control;
use error;
use layout::ComponentIndex;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventMessage {
    // pub src_name: String,
    pub idx: ComponentIndex,
    pub event: Event
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    ButtonClick(control::button::ButtonClick),
    SliderChange(control::slider::SliderChange)
}
impl Event {
    pub fn name(&self) -> &str {
        match *self {
            Event::ButtonClick(_) => "ButtonClick",
            Event::SliderChange(_) => "SliderChange",
        }
    }
}

pub trait HandleEvent<St> {
    fn handle_event(&self, event: Event, state: St) -> error::Result<St>;
}

pub trait EventHandler<Payload, St>: Send + Sync + RefUnwindSafe {
    fn handle(&self, payload: Payload, state: St) -> error::Result<St>;
}
impl<Payload, St, F> EventHandler<Payload, St> for F
    where F: Fn(Payload, St) -> error::Result<St> + Send + Sync + RefUnwindSafe
{
    fn handle(&self, payload: Payload, state: St) -> error::Result<St> {
        self(payload, state)
    }
}
