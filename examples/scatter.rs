extern crate rhubarb;
extern crate rhubarb_graph as rg;
extern crate agnes;

use std::path::Path;

use agnes::source::{CsvReader, CsvSource};
use agnes::DataView;

use rhubarb::error::*;
use rg::{Graph};
use rg::color::Color;
use rg::scatter::{Scatter};
use rg::common::Mode;

fn generate_scatter() -> Result<Graph<Scatter>> {

    let data_filepath = Path::new(file!()).parent().unwrap()
        .join("../rhubarb-graph/tests/data/sample1.csv");
    let mut csv_rdr = CsvReader::new(
        &CsvSource::new(data_filepath.into()).unwrap()
    ).unwrap();
    let dv: DataView = csv_rdr.read()?.into();

    let scatter = Scatter::new(
            dv.v("val1"),
            dv.v("val2")
        )
        .marker(
            rg::common::Marker::default()
                // .sizes(rg::SingleOrMore::More((dv.v("val3")).as_fieldview().unwrap()))
                .color(Color::new(1.0, 0.0, 0.0))
        )
        .text(dv.v("state"))
        .mode(Mode::Markers);

    let graph: rg::Graph<Scatter> = rg::Graph::new(
        vec![scatter],
        rg::Layout::default()
            .title("Sample Scatter Plot!")
            .width(600usize)
            .height(500usize)
            .x_axis(rg::common::Axis::default()
                .title("Value 1")
                .kind(rg::common::AxisKind::Linear)
            )
            .y_axis(rg::common::Axis::default()
                .title("Value 2")
                .kind(rg::common::AxisKind::Linear)
            )
            .margin(rg::common::Margin::from_hv(40, 40))
    );
    Ok(graph)
}

pub fn main() {
    //TODO: update this
    // rhubarb::start(generate_scatter);
}
