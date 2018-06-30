use std::sync::Arc;
use std::fmt;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use control::Control;
use event::{HandleEvent, Event, EventHandler};
use error;

#[derive(Clone)]
pub struct ButtonControl<St> {
    text: String,
    on_click: Option<Arc<EventHandler<ButtonClick, St>>>
}
impl<St> fmt::Debug for ButtonControl<St> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ButtonControl")
            .field("text", &self.text)
            .field("on_click", &match self.on_click {
                Some(_) => "<<on_click handler>>",
                None => "none"
            }.to_string())
            .finish()
    }
}
impl<St> ButtonControl<St> {
    pub fn new<S: AsRef<str>>(text: S) -> ButtonControl<St> {
        ButtonControl {
            text: text.as_ref().into(),
            on_click: None
        }
    }
    pub fn with_on_click<E: 'static + EventHandler<ButtonClick, St>>(self, f: E)
        -> ButtonControl<St>
    {
        ButtonControl {
            text: self.text,
            on_click: Some(Arc::new(f))
        }
    }
}
impl<St> HandleEvent<St> for ButtonControl<St> {
    fn handle_event(&self, event: Event, state: St) -> error::Result<St> {
        match event {
            Event::ButtonClick(click_details) => {
                match self.on_click {
                    Some(ref handler) => handler.handle(click_details, state),
                    None => Ok(state)
                }
            },
            _ => {
                return Err(error::RhubarbError::InvalidEvent {
                    event,
                    component_type: "ButtonControl".into()
                })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonClick {}

impl<St> Serialize for ButtonControl<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("ButtonControl", 1)?;
        state.serialize_field("text", &self.text)?;
        state.end()
    }
}
impl<St> From<ButtonControl<St>> for Control<St> {
    fn from(btn: ButtonControl<St>) -> Control<St> { Control::Button(btn) }
}
