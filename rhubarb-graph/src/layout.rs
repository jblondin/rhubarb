use serde::ser::{Serialize, Serializer, SerializeStruct};

use CountExistFields;
use common::{Axis, Margin, Legend};

#[derive(Debug, Clone, Default, GraphElem)]
pub struct Layout {
    title: Option<String>,
    width: Option<usize>,
    height: Option<usize>,
    #[graphelem(serialize(name = "xaxis"))]
    x_axis: Option<Axis>,
    #[graphelem(serialize(name = "yaxis"))]
    y_axis: Option<Axis>,
    margin: Option<Margin>,
    showlegend: Option<bool>,
    legend: Option<Legend>,
    autosize: Option<bool>
}
