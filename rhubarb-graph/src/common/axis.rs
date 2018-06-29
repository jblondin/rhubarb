use serde::ser::{Serialize, Serializer, SerializeStruct};

use CountExistFields;

#[derive(Debug, Clone)]
pub enum AxisKind {
    Linear,
    Log
}
impl Serialize for AxisKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            AxisKind::Linear => serializer.serialize_str("linear"),
            AxisKind::Log    => serializer.serialize_str("log"),
        }
    }
}

#[derive(Debug, Clone, Default, GraphElem)]
pub struct Axis {
    title: Option<String>,
    #[graphelem(serialize(name = "type"))]
    kind: Option<AxisKind>
}
