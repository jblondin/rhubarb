use serde::ser::{Serialize, Serializer, SerializeMap};

use agnes::DataView;

use common::{Modes, Marker};
use CountExistFields;

#[derive(Debug, Clone, Default)]
pub struct Scatter {
    x: DataView,
    y: DataView,
    mode: Option<Modes>,
    marker: Option<Marker>,
    text: Option<DataView>,
    opacity: Option<f64>,
    name: Option<String>
}

impl Scatter {
    pub fn new(x: DataView, y: DataView) -> Scatter {
        Scatter {
            x: x,
            y: y,
            mode: None,
            marker: None,
            text: None,
            opacity: None,
            name: None,
        }
    }
    pub fn mode<T: Into<Modes>>(mut self, t: T) -> Scatter {
        self.mode = Some(t.into());
        self
    }
    pub fn marker<T: Into<Marker>>(mut self, t: T) -> Scatter {
        self.marker = Some(t.into());
        self
    }
    pub fn text<T: Into<DataView>>(mut self, t: T) -> Scatter {
        self.text = Some(t.into());
        self
    }
    pub fn opacity(mut self, opacity: f64) -> Scatter {
        self.opacity = Some(opacity);
        self
    }
    pub fn name<T: AsRef<str>>(mut self, name: T) -> Scatter {
        self.name = Some(name.as_ref().to_string());
        self
    }
}

impl CountExistFields for Scatter {
    fn count_existing_fields(&self) -> usize {
        let mut count = 0;
        if self.x.as_fieldview().is_some() { count += 1; }
        if self.y.as_fieldview().is_some() { count += 1; }
        if self.mode.is_some() { count += 1; }
        if self.marker.is_some() { count += 1; }
        if let Some(ref text) = self.text {
            if text.as_fieldview().is_some() { count += 1; }
        }
        if self.name.is_some() { count += 1; }
        if self.opacity.is_some() { count += 1; }
        count
    }
}

impl Serialize for Scatter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(self.count_existing_fields()))?;
        if let Some(ref x) = self.x.as_fieldview() { map.serialize_entry("x", x)?; }
        if let Some(ref y) = self.y.as_fieldview() { map.serialize_entry("y", y)?; }
        if let Some(ref mode) = self.mode { map.serialize_entry("mode", &mode.as_string())? ; }
        if let Some(ref marker) = self.marker { map.serialize_entry("marker", &marker)?; }
        if let Some(ref text) = self.text {
            if let Some(ref text) = text.as_fieldview() { map.serialize_entry("text", text)?; }
        }
        if let Some(opacity) = self.opacity { map.serialize_entry("opacity", &opacity)?; }
        if let Some(ref name) = self.name { map.serialize_entry("name", &name)?; }
        map.end()
    }
}
