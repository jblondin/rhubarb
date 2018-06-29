use std::panic::RefUnwindSafe;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use {Chart, ChartState};
use layout::{Component, IndexedComponent, ComponentIndex};
use event::EventMessage;
use error;
use layout::Layout;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage<St> {
    pub ui_state: St,
    pub event_message: EventMessage,
}

pub struct LayoutUpdate<'a, St: 'a> {
    update: Update<St>,
    layout: &'a Layout<St>
}
impl<'a, St> LayoutUpdate<'a, St> {
    pub fn new<U: Into<Update<St>>>(update: U, layout: &'a Layout<St>) -> LayoutUpdate<St>
    {
        LayoutUpdate {
            update: update.into(),
            layout
        }
    }
}
impl<'a, St: ChartState> Serialize for LayoutUpdate<'a, St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = match self.update.chart {
            Some(ref chart) => {
                let mut state = serializer.serialize_struct("Update", 4)?;
                state.serialize_field("chart", &chart)?;
                state
            },
            None => {
                serializer.serialize_struct("Update", 3)?
            }
        };
        state.serialize_field("layout", &self.layout)?;
        state.serialize_field("components", &self.update.components)?;
        state.serialize_field("state", &self.update.state)?;
        state.end()
    }
}


pub struct Update<St> {
    chart: Option<Chart>,
    // updates to referenced values in layout
    components: Vec<IndexedComponent<St>>,
    // state communicated to client
    state: St
}
// impl<St> From<Chart> for Update<St> {
//     fn from(chart: Chart) -> Update<St> {
//         Update {
//             chart: Some(chart),
//             components: vec![],
//         }
//     }
// }
// impl<St> From<Option<Chart>> for Update<St> {
//     fn from(chart: Option<Chart>) -> Update<St> {
//         Update {
//             chart,
//             components: vec![],
//         }
//     }
// }
impl<St> Update<St> {
    pub fn new<C: Into<Option<Chart>>>(chart: C, state: St) -> Update<St> {
        Update {
            chart: chart.into(),
            components: vec![],
            state
        }
    }
    // pub fn empty() -> Update<St> {
    //     Update {
    //         chart: None,
    //         components: vec![],
    //     }
    // }
    pub fn set_chart(&mut self, chart: Chart) {
        self.chart = Some(chart);
    }
    pub fn add_component<C: Into<Component<St>>>(&mut self, component_idx: ComponentIndex,
        updated_component: C) -> error::Result<()>
    {
        self.components.push(IndexedComponent {
            // idx: layout.get_component_index(component_name.as_ref()).ok_or(
            //     error::RhubarbError::ComponentRegistry(component_name.as_ref().into())
            // )?,
            idx: component_idx,
            component: updated_component.into()
        });
        Ok(())
    }
}
impl<St: ChartState> Serialize for Update<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = match self.chart {
            Some(ref chart) => {
                let mut state = serializer.serialize_struct("Update", 2)?;
                state.serialize_field("chart", &chart)?;
                state
            },
            None => {
                serializer.serialize_struct("Update", 1)?
            }
        };
        state.serialize_field("components", &self.components)?;
        state.serialize_field("state", &self.state)?;
        state.end()
    }
}

pub trait GenerateUpdate<St>: Send + Sync + RefUnwindSafe + Clone {
    fn update(&self, layout: &Layout<St>, prev_state: Option<St>, state: St)
        -> error::Result<Update<St>>;
}
impl<St, F> GenerateUpdate<St> for F
    where F: Fn(&Layout<St>, Option<St>, St) -> error::Result<Update<St>>
        + Send + Sync + RefUnwindSafe + Clone
{
    fn update(&self, layout: &Layout<St>, prev_state: Option<St>, state: St)
        -> error::Result<Update<St>>
    {
        self(layout, prev_state, state)
    }
}


