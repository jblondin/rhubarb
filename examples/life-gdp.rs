#[macro_use] extern crate serde_derive;
extern crate rhubarb;
extern crate rhubarb_graph as rg;
extern crate agnes;

use agnes::apply::{ApplyUnchecked, Select, Unique};
use agnes::{DataView, MaybeNa, Filter};
use agnes::join::{Join, JoinKind};

use rhubarb::{Chart, RhubarbApp};
use rhubarb::error::*;
use rhubarb::update::Update;
use rhubarb::layout::{Layout, Component, ComponentIndex};
use rhubarb::control::button::{ButtonControl, ButtonClick};
use rhubarb::control::slider::{SliderControl, SliderChange};
use rhubarb::control::Control;

use rg::scatter::Scatter;
use rg::common::{Line, Mode, Marker};
use rg::color::name as cname;

mod common;

fn load_gdp_life_csv(years: &Vec<&str>) -> Result<DataView> {

    // load the GDP data set from CSV
    let mut csv_rdr = common::load_csv("gdp/API_NY.GDP.MKTP.CD_DS2_en_csv_v2.csv")?;
    // we only care about the country name, country code, and 1987 / 2006 GDP
    let mut fields = vec!["Country Name", "Country Code"];
    fields.extend(years.iter());
    let dv_gdp = DataView::from(csv_rdr.read()?).v(fields);

    // load the country metadata details from CSV, so we can filter out the non-country records from
    // the GDP and life expectancy data sets
    let mut csv_rdr = common::load_csv("gdp/Metadata_Country_API_NY.GDP.MKTP.CD_DS2_en_csv_v2.csv")?;
    let mut dv_gdp_metadata = DataView::from(csv_rdr.read()?)
        .v(["Country Code", "Region"]);
    // so, all the non-country records have a missing value for 'Region', so we can just filter for
    // any that have a valid string value
    dv_gdp_metadata.filter(&"Region".into(), |_: &String| true).unwrap();

    // perform an inner join the GDP with the filtered metadata, which will give us a table with
    // the region and GDPs of all the countries (and not the extraneous non-country records)
    let mut dv_gdp_joined: DataView = dv_gdp.join(&dv_gdp_metadata, Join::equal(
        JoinKind::Inner,
        "Country Code",
        "Country Code"
    ))?.into();

    // load the life expectancy data set from CSV
    let mut csv_rdr = common::load_csv("life/API_SP.DYN.LE00.IN_DS2_en_csv_v2.csv")?;
    // since we are going to join on the country code, and we already have the country name from
    // the GDP data set, we just need the country code and the 2006 life expectancy.
    let mut fields = vec!["Country Code"];
    fields.extend(years.iter());
    let mut dv_life = DataView::from(csv_rdr.read()?).v(fields);

    // rename the '2006' columns from the gdp and life expectancy data sets so the names don't
    // collide when we try to join them.
    let mut final_fields: Vec<String> = vec!["Country Name".into(), "Region".into()];
    for year in years {
        let year_gdp = format!("{} GDP", year);
        dv_gdp_joined.rename(*year, year_gdp.clone())?;
        final_fields.push(year_gdp);
        let year_le = format!("{} Life Expectancy", year);
        dv_life.rename(*year, year_le.clone())?;
        final_fields.push(year_le);
    }

    // perform an inner join between the GDP data set (with non-countries filtered out and with
    // the region included)
    let dv: DataView = dv_gdp_joined.join(&dv_life, Join::equal(
        JoinKind::Inner,
        "Country Code",
        "Country Code"
    ))?.into();
    // we no longer need the country name, so just return a DataView with everything else
    Ok(dv.v(final_fields))
}

fn generate_region_traces(dv: &DataView, year: &str) -> Result<Vec<Scatter>> {
    // we need a scatter trace for each region, so let's first get a list of the regions
    let regions = dv.unique("Region")?;

    // create the vector to store the scatter traces into
    let mut scatters = vec![];

    regions
        // we only have one field in this dataview, but we need to select it
        .select(&"Region".into())
        // and we apply a function on each record in this field
        .apply_unchecked(
            // with this state (which will be updated as we iterate through the field)
            (dv, &mut scatters),
            |region: MaybeNa<&String>, state| {
                // extract the state
                let (dv, scatters) = state;

                // start by creating a new view into the data set
                let mut subdv = dv.clone();
                // filter this view with the current region
                subdv.filter(&"Region".into(), |val: &String| {
                    match region {
                        MaybeNa::Exists(region) => val == region,
                        MaybeNa::Na => false,
                    }
                }).expect("Invalid filter");
                if subdv.nrows() > 0 {
                    // build the new Scatter trace for this region
                    scatters.push(Scatter::new(
                            subdv.v(format!("{} GDP", year)),
                            subdv.v(format!("{} Life Expectancy", year))
                        )
                        .text(subdv.v("Country Name"))
                        .mode(Mode::Markers)
                        .marker(
                            Marker::default()
                                .size(15.0)
                                .line(
                                    Line::default()
                                        .width(0.5)
                                        .color(cname::white())
                                )
                        )
                        .opacity(0.7)
                        .name(region.unwrap()))
                } else {
                    panic!("attempted to add scatter on missing value")
                };
            }
        )?;
    // and return all the built scatter traces
    Ok(scatters)
}

fn generate_update(dv: &DataView, years: &Vec<&str>, slider_index: ComponentIndex,
    layout: &Layout<UiState>, prev_state: Option<UiState>, state: UiState)
    -> Result<Update<UiState>>
{
    let update_chart = if let Some(prev_state) = prev_state {
        prev_state.year_idx != state.year_idx
    } else {
        true
    };
    let chart = if update_chart {
        let scatter_traces = generate_region_traces(dv, years[state.year_idx])?;

        Some(Chart::Scatter(rg::Graph::new(
            scatter_traces,
            rg::Layout::default()
                .title(format!("Life Expectancy and GDP, {}", years[state.year_idx]))
                .width(800usize)
                .height(500usize)
                .autosize(true)
                .x_axis(rg::common::Axis::default()
                    .title("GDP")
                    .kind(rg::common::AxisKind::Log)
                )
                .y_axis(rg::common::Axis::default()
                    .title("Life Expectancy")
                    .kind(rg::common::AxisKind::Linear)
                )
                .margin(rg::common::Margin::from_hv(40, 40))
                .legend(rg::common::Legend::default()
                    .bg_color(cname::white())
                    .border_color(cname::black())
                    .border_width(1)
                    .x(0.0).y(1.0)
                )
                .showlegend(true)
        )))
    } else {
        None
    };
    let mut new_slider = layout[slider_index].clone();
    if let Component::Control(Control::Slider(ref mut slider)) = new_slider {
        slider.curr_value = state.year_idx.clone();
    } else {
        panic!("unexpected component at slider index");
    };

    let mut update = Update::new(chart, state);
    update.add_component(slider_index, new_slider)?;

    Ok(update)
}

// fn generate_life_gdp(dv: &DataView, years: &Vec<&str>, state: UiState)
//     -> Result<Container<Scatter>>
// {
//     println!("generating with state {:#?}", state);
//     let UiState { year_idx } = state;

//     let scatter_traces = generate_region_traces(dv, years[year_idx])?;

//     let graph: rg::Graph<Scatter> = rg::Graph::new(
//         scatter_traces,
//         rg::Layout::default()
//             .title(format!("Life Expectancy and GDP, {}", years[year_idx]))
//             .width(800usize)
//             .height(500usize)
//             .autosize(true)
//             .x_axis(rg::common::Axis::default()
//                 .title("GDP")
//                 .kind(rg::common::AxisKind::Log)
//             )
//             .y_axis(rg::common::Axis::default()
//                 .title("Life Expectancy")
//                 .kind(rg::common::AxisKind::Linear)
//             )
//             .margin(rg::common::Margin::from_hv(40, 40))
//             .legend(rg::common::Legend::default()
//                 .bg_color(cname::white())
//                 .border_color(cname::black())
//                 .border_width(1)
//                 .x(0.0).y(1.0)
//             )
//             .showlegend(true)
//     );
//     let mut container = Container::from(graph);

//     let next_year_idx = (year_idx + 1) % years.len();
//     let btn_text = years[next_year_idx];
//     container.add_control(ButtonControl::new(btn_text)
//         // .with_action(Effect::new("year_idx", StateValue::value(format!("{}", next_year_idx))))
//     );
//     // container.add_control(WithAction::<action::Change>::with_action(
//     //     SliderControl::new(years.clone(), year_idx),
//     //     Effect::new("year_idx", StateValue::Selected)
//     // ));
//         // .with_on_click(btn_payload));
//     // container.add_control(SliderControl::new(vec!["1987", "2006"]));

//     container.add_control(ButtonControl::new(btn_text)
//         .with_on_click(|state: UiState| -> UiState {  })
//     );
//     Ok(container)
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UiState {
    year_idx: usize,
}
impl Default for UiState {
    fn default() -> UiState {
        UiState { year_idx: 0 }
    }
}

pub fn main() -> Result<()> {
    let years = vec!["1975", "1985", "1995", "2005", "2015"];
    let years_len = years.len();
    let dv = load_gdp_life_csv(&years).expect("error loading data sets");
    // rhubarb::start(move |state: UiState| generate_life_gdp(&dv, &years, state));

    let mut layout: Layout<UiState> = Layout::default();
    let main_panel = layout.add_panel("main_panel", None)?;
    layout.add_control_to_panel("year_button",
        ButtonControl::new("Next").with_on_click(
            move |_: ButtonClick, mut state: UiState| -> Result<UiState> {
                state.year_idx = (state.year_idx + 1) % years_len;
                Ok(state)
            }
        ),
        main_panel
    )?;
    let slider_index = layout.add_control_to_panel("year_slider",
        SliderControl::new(years.clone(), 0).with_on_change(
            |change_event: SliderChange, mut state: UiState| -> Result<UiState> {
                state.year_idx = change_event.idx;
                Ok(state)
            }
        ),
        main_panel
    )?;

    let app = RhubarbApp::new(
        layout,
    );

    app.start(move |layout: &Layout<UiState>, prev_state: Option<UiState>, state: UiState|
        generate_update(&dv, &years, slider_index, layout, prev_state, state)
    );
    Ok(())
    // let app = RhubarbApp::new(
    //     chart,
    //     move |prev_state: UiState, state: UiState| generate_life_gdp(&dv, &years, state)
    // );
}
