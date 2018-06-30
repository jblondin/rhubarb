use std::collections::HashMap;
use std::ops::Index;

use serde::{Serialize, Serializer};
use serde::ser::{SerializeStruct, SerializeSeq};

use ChartState;
use control::Control;
use event::{Event, EventMessage, HandleEvent};
use error;

pub type ComponentIndex = usize;
type ComponentRegistry = HashMap<String, ComponentIndex>;

#[derive(Debug, Clone)]
pub enum Component<St> {
    Control(Control<St>),
    Panel(Panel),
}
impl<St> HandleEvent<St> for Component<St> {
    fn handle_event(&self, event: Event, state: St) -> error::Result<St> {
        match *self {
            Component::Control(ref ctrl) => {
                ctrl.handle_event(event, state)
            },
            Component::Panel(ref panel) => {
                panel.handle_event(event, state)
            }
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Panel {
    children: Vec<ComponentIndex>
}
impl Panel {
    pub fn new() -> Panel {
        Panel { children: vec![] }
    }
    pub fn add_child(&mut self, component_idx: ComponentIndex) {
        self.children.push(component_idx);
    }
}
impl<St> HandleEvent<St> for Panel {
    fn handle_event(&self, event: Event, _: St) -> error::Result<St> {
        Err(error::RhubarbError::InvalidEvent { event, component_type: "panel".into() })
    }
}

impl<St> From<Panel> for Component<St> {
    fn from(panel: Panel) -> Component<St> { Component::Panel(panel) }
}
impl<St> From<Control<St>> for Component<St> {
    fn from(control: Control<St>) -> Component<St> { Component::Control(control) }
}

#[derive(Debug)]
pub struct Layout<St> {
    component_store: ComponentStore<St>,
    registry: ComponentRegistry,
    children: Vec<ComponentIndex>,
}

impl<St> Default for Layout<St> {
    fn default() -> Layout<St> {
        Layout {
            component_store: ComponentStore(Vec::new()),
            registry: ComponentRegistry::new(),
            children: Vec::new(),
        }
    }
}
impl<St> Layout<St> {
    pub fn new() -> Layout<St> { Layout::default() }

    pub fn add_panel<S: AsRef<str>>(&mut self, name: S,
        parent: Option<ComponentIndex>) -> error::Result<ComponentIndex>
    {
        let name = name.as_ref().to_string();
        let new_idx = self.add_component(name.clone(), Panel::new());
        match parent {
            Some(parent_idx) => {
                self.add_to_panel(name, new_idx, parent_idx)?;
            },
            None => {
                self.children.push(new_idx);
            }
        }
        Ok(new_idx)
    }

    pub fn add_control_to_panel<S: AsRef<str>, C: Into<Control<St>>>(
        &mut self, name: S, control: C, panel_idx: ComponentIndex) -> error::Result<ComponentIndex>
    {
        let name = name.as_ref().to_string();
        let control_idx = self.add_component(name.clone(), control.into());
        self.add_to_panel(name, control_idx, panel_idx)?;
        Ok(control_idx)
    }

    pub fn get_component_index<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        self.registry.get(name.as_ref()).cloned()
    }

    fn add_to_panel<S: AsRef<str>>(&mut self, name: S, component_idx: ComponentIndex,
        panel_idx: ComponentIndex) -> error::Result<()>
    {
        match self.component_store.0[panel_idx] {
            Component::Panel(ref mut panel) => {
                panel.add_child(component_idx);
            },
            _ => {
                return Err(error::RhubarbError::InvalidLayout(
                    format!("unable able to add component {} to non-panel with index {}",
                        name.as_ref(), panel_idx)));
            }
        }
        Ok(())
    }
    fn add_component<S: AsRef<str>, C: Into<Component<St>>>(&mut self, name: S, component: C)
        -> ComponentIndex
    {
        let new_idx = self.component_store.0.len();
        self.component_store.0.push(component.into());
        self.registry.entry(name.as_ref().into()).or_insert(new_idx);
        new_idx
    }

    pub fn handle_event(&self, event: EventMessage, state: St) -> error::Result<St> {
        self.component_store.0[event.idx].handle_event(event.event, state)
    }
}
impl<St> Index<ComponentIndex> for Layout<St> {
    type Output = Component<St>;
    fn index(&self, index: ComponentIndex) -> &Component<St> {
        &self.component_store[index]
    }
}
impl<St: ChartState> Serialize for Layout<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Layout", 2)?;
        state.serialize_field("component_store", &self.component_store)?;
        state.serialize_field("children", &self.children)?;
        state.end()
    }
}

#[derive(Debug, Clone)]
struct ComponentStore<St>(Vec<Component<St>>);
impl<St: ChartState> Serialize for ComponentStore<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (idx, component) in self.0.iter().enumerate() {
            seq.serialize_element(&IndexedComponentRef { idx, component })?;
        }
        seq.end()
    }
}
impl<St> Index<ComponentIndex> for ComponentStore<St> {
    type Output = Component<St>;
    fn index(&self, index: ComponentIndex) -> &Component<St> {
        &self.0[index]
    }
}

#[derive(Debug)]
struct IndexedComponentRef<'a, St: 'a> {
    idx: ComponentIndex,
    component: &'a Component<St>
}

pub struct IndexedComponent<St> {
    pub idx: ComponentIndex,
    pub component: Component<St>
}

macro_rules! impl_indexed_component_ser {
    ($name:tt ($($deref:tt)*) ($($lifetime:tt)*)) => {

impl<$($lifetime,)* St: ChartState> Serialize for $name<$($lifetime,)* St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("IndexedComponent", 2)?;
        state.serialize_field("idx", &self.idx)?;
        match $($deref)*self.component {
            Component::Control(ref ctrl) => {
                state.serialize_field("control", ctrl)?;
            },
            Component::Panel(ref panel) => {
                state.serialize_field("panel", panel)?;
            }
        }
        state.end()
    }
}

    }
}
impl_indexed_component_ser!(IndexedComponent () ());
impl_indexed_component_ser!(IndexedComponentRef (*) ('a));
