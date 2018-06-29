use serde::ser::{Serialize, Serializer, SerializeStruct};

use Layout;

#[derive(Debug, Clone)]
pub struct Graph<D> {
    data: Vec<D>,
    layout: Layout
}
impl<D> Graph<D> {
    pub fn new<V: Into<Vec<D>>, L: Into<Layout>>(data: V, layout: L) -> Graph<D> {
        Graph {
            data: data.into(),
            layout: layout.into()
        }
    }
}

impl<D: Serialize + Default> Serialize for Graph<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Graph", 2)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("layout", &self.layout)?;
        state.end()
    }
}
