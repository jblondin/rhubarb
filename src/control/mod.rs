use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

// mod event;
// pub use self::event::ClickEventHandler;
// pub mod action;

pub mod button;
pub mod slider;

use event::{Event, HandleEvent};
use error;
// pub trait Control {
//     fn ctrl_type(&self) -> &str;
//     fn with_action<T>(&self, effect: action::Effect<T>) -> action::ActionControl<Self, T>;
//     fn add_to_serialized_struct<S: Serializer>(&self, state: &mut S::SerializeStruct);
// }

// pub struct Control {
//     inner: ControlInner,
//     action: Box<action::Action>,
// }

// pub enum ControlInner {
//     Button(ButtonControl),
//     Slider(SliderControl),
// }

#[derive(Debug, Clone)]
pub enum Control<St> {
    Button(button::ButtonControl<St>),
    Slider(slider::SliderControl<St>),
}
impl<St> Serialize for Control<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Control", 2)?;
        // state.serialize_field("type", self.ctrl_type())?;
        match *self {
            Control::Button(ref btn) => {
                state.serialize_field("type", "button")?;
                state.serialize_field("properties", btn)?;
            }
            Control::Slider(ref slider) => {
                state.serialize_field("type", "slider")?;
                state.serialize_field("properties", slider)?;
            }
        }
        state.end()
    }
}
impl<St> HandleEvent<St> for Control<St> {
    fn handle_event(&self, event: Event, state: St) -> error::Result<St> {
        match *self {
            Control::Button(ref btn) => {
                btn.handle_event(event, state)
            },
            Control::Slider(ref slider) => {
                slider.handle_event(event, state)
            }
        }
    }
}

