use std::sync::Arc;
use std::fmt;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use event::{Event, EventHandler, HandleEvent};
use error;
use control::Control;

#[derive(Clone)]
pub struct SliderControl<St> {
    values: Vec<String>,
    pub curr_value: usize,
    on_change: Option<Arc<EventHandler<SliderChange, St>>>
}
impl<St> fmt::Debug for SliderControl<St> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SliderControl")
            .field("values", &self.values)
            .field("curr_value", &self.curr_value)
            .field("on_change", &match self.on_change {
                Some(_) => "<<on_change handler>>",
                None => "none"
            }.to_string())
            .finish()
    }
}
impl<St> SliderControl<St> {
    pub fn new<S: AsRef<str>>(values: Vec<S>, curr_value: usize) -> SliderControl<St> {
        SliderControl {
            values: values.iter().map(|s| s.as_ref().into()).collect(),
            curr_value: curr_value,
            on_change: None
        }
    }
    pub fn with_on_change<E: 'static + EventHandler<SliderChange, St>>(self, f: E)
        -> SliderControl<St>
    {
        SliderControl {
            values: self.values,
            curr_value: self.curr_value,
            on_change: Some(Arc::new(f))
        }
    }
}
impl<St> HandleEvent<St> for SliderControl<St> {
    fn handle_event(&self, event: Event, state: St) -> error::Result<St> {
        match event {
            Event::SliderChange(change_details) => {
                match self.on_change {
                    Some(ref handler) => handler.handle(change_details, state),
                    None => Ok(state)
                }
            }
            _ => {
                return Err(error::RhubarbError::InvalidEvent {
                    event,
                    component_type: "SliderControl".into()
                })
            }
        }
    }
}

impl<St> Serialize for SliderControl<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("SliderControl", 2)?;
        state.serialize_field("values", &self.values)?;
        state.serialize_field("curr_value", &self.curr_value)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SliderChange {
    pub idx: usize
}

impl<St> From<SliderControl<St>> for Control<St> {
    fn from(slider: SliderControl<St>) -> Control<St> { Control::Slider(slider) }
}
