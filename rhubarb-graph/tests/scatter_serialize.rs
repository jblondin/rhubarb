extern crate agnes;
extern crate serde;
extern crate serde_json;
extern crate rhubarb_graph as rg;

use rg::scatter::Scatter;
use std::path::Path;

use agnes::source::{CsvReader, CsvSource};
use agnes::view::DataView;

#[test]
fn scatter_serialize() {
    let data_filepath = Path::new("../").join(
        Path::new(file!()).parent().unwrap().join("data/sample1.csv")
    );
    let source = CsvSource::new((&data_filepath).into()).unwrap();
    let mut csv_rdr = CsvReader::new(&source).unwrap();
    let dv: DataView = DataView::from(csv_rdr.read().unwrap()).v(["state", "val1", "val2"]);

    println!("{}", dv);
    println!("{}", serde_json::to_string(&dv).unwrap());
    let scatter = Scatter::new(
        dv.v("val1"),
        dv.v("val2")
    );
    println!("{}", serde_json::to_string(&scatter).unwrap());

    let graph: rg::Graph<Scatter> = rg::Graph::new(
        vec![scatter],
        rg::Layout::default()
            .title("Sample Scatter Plot!")
            .width(600usize)
            .height(500usize)
            .x_axis(rg::common::Axis::default()
                .title("Value 1")
                .kind(rg::common::AxisKind::Log)
            )
            .y_axis(rg::common::Axis::default()
                .title("Value 2")
                .kind(rg::common::AxisKind::Linear)
            )
            .margin(rg::common::Margin::from_hv(10, 10))
    );
    println!("{}", serde_json::to_string_pretty(&graph).unwrap());
}
