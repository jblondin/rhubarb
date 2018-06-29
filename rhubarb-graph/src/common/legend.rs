use serde::ser::{Serialize, Serializer, SerializeStruct};

use CountExistFields;
use color::Color;

#[derive(Debug, Clone, Default, GraphElem)]
pub struct Legend {
    #[graphelem(serialize(name = "bgcolor"))]
    bg_color: Option<Color>,
    #[graphelem(serialize(name = "bordercolor"))]
    border_color: Option<Color>,
    #[graphelem(serialize(name = "borderwidth"))]
    border_width: Option<i32>,
    x: Option<f32>,
    y: Option<f32>
}
