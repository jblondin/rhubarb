#[macro_use] extern crate df;
extern crate serde;
extern crate serde_json;
extern crate rhubarb_graph as rg;

use std::path::Path;

use df::source::{CsvReader, CsvSourceBuilder, FileSource};
use df::field::{FieldIdent, FieldType};
use df::view::DataView;

#[test]
fn scatter_serialize() {
    let data_filepath = Path::new("../").join(
        Path::new(file!()).parent().unwrap().join("data/sample1.csv")
    );
    let file = FileSource::new((&data_filepath).into());
    let mut csv_rdr = CsvReader::new(
        CsvSourceBuilder::default()
            .file_source(file)
            .fields(fields![
                "state" => FieldType::Text,
                "val1" => FieldType::Unsigned,
                "val2" => FieldType::Float
            ])
            .build().unwrap()
    ).unwrap();
    let dv: DataView = csv_rdr.read().unwrap().into();

    println!("{}", dv);
    println!("{}", serde_json::to_string(&dv).unwrap());
    let scatter = rg::ScatterBuilder::default()
        .x(&dv%"val1")
        .y(&dv%"val2")
        .build().unwrap();
    println!("{}", serde_json::to_string(&scatter).unwrap());

    let graph: rg::Graph<rg::Scatter> = rg::GraphBuilder::default()
        .data(vec![scatter])
        .layout(rg::LayoutBuilder::default()
            .title("Sample Scatter Plot!")
            .width(600usize)
            .height(500usize)
            .x_axis(rg::AxisBuilder::default()
                .title("Value 1")
                .kind(rg::AxisKind::Log)
                .build().unwrap()
            )
            .y_axis(rg::AxisBuilder::default()
                .title("Value 2")
                .kind(rg::AxisKind::Linear)
                .build().unwrap()
            )
            .margin(rg::Margin::from_hv(10, 10))
            .build().unwrap()
        ).build().unwrap();
    println!("{}", serde_json::to_string_pretty(&graph).unwrap());
}
