use serde::ser::{Serialize, Serializer, SerializeStruct};

use CountExistFields;
use color::Color;
use SingleOrMore;
use agnes::view::FieldView;

#[derive(Debug, Clone, Default, GraphElem)]
pub struct Line {
    width: Option<f64>,
    color: Option<Color>,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Lines,
    Markers,
    Text,
}
impl Mode {
    fn as_string(&self) -> String {
        match *self {
            Mode::Lines => "lines",
            Mode::Markers => "markers",
            Mode::Text => "text",
        }.to_string()
    }
}
#[derive(Clone, Debug)]
pub struct Modes(Vec<Mode>);
impl Modes {
    pub fn as_string(&self) -> String {
        self.0.iter().map(|mode| mode.as_string()).collect::<Vec<_>>()[..].join("+")
    }
}

impl From<Mode> for Modes {
    fn from(mode: Mode) -> Modes {
        Modes(vec![mode])
    }
}
impl From<Vec<Mode>> for Modes {
    fn from(modes: Vec<Mode>) -> Modes {
        Modes(modes)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Margin {
    pub l: usize,
    pub r: usize,
    pub t: usize,
    pub b: usize,
}

impl Margin {
    pub fn new(l: usize, r: usize, t: usize, b: usize) -> Margin {
        Margin {
            l: l, r: r, t: t, b: b
        }
    }
    pub fn from_hv(horiz: usize, vert: usize) -> Margin {
        Margin {
            l: horiz, r: horiz,
            t: vert, b: vert
        }
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Circle,
    CircleOpen,
    Square,
    SquareOpen,
    X,
    XOpen,
}
impl Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            Symbol::Circle => serializer.serialize_unit_variant("Symbol", 0, "circle"),
            Symbol::CircleOpen => serializer.serialize_unit_variant("Symbol", 0, "circle-open"),
            Symbol::Square => serializer.serialize_unit_variant("Symbol", 0, "square"),
            Symbol::SquareOpen => serializer.serialize_unit_variant("Symbol", 0, "square-open"),
            Symbol::X => serializer.serialize_unit_variant("Symbol", 0, "x"),
            Symbol::XOpen => serializer.serialize_unit_variant("Symbol", 0, "x-open"),
        }
    }
}

#[derive(Debug, Clone, Default, GraphElem)]
pub struct Marker {
    symbol: Option<Symbol>,
    color: Option<Color>,
    line: Option<Line>,
    size: Option<SingleOrMore<f64, FieldView>>
}
