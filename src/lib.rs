extern crate agnes;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate handlebars;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rhubarb_graph;
extern crate unicase;

pub mod control;
pub mod handler;
pub mod router;
pub mod resource;
pub mod error;
pub mod logger;
pub mod layout;
pub mod event;
pub mod update;

// use std::collections::HashMap;

use std::sync::Arc;
use std::panic::RefUnwindSafe;

use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;
// use serde::ser::SerializeStruct;

use rhubarb_graph as rg;
// use rhubarb_graph::Graph;

// use control::Control;
use layout::Layout;
use update::GenerateUpdate;

#[derive(Debug)]
pub enum Chart {
    Scatter(rg::Graph<rg::scatter::Scatter>)
}
impl Serialize for Chart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            Chart::Scatter(ref graph) => graph.serialize(serializer)
        }
    }
}

pub trait ChartState:
    'static + Send + Sync + RefUnwindSafe + Serialize + DeserializeOwned + Default + Clone {}
impl<T> ChartState for T where T:
    'static + Send + Sync + RefUnwindSafe + Serialize + DeserializeOwned + Default + Clone {}


pub struct RhubarbApp<St> {
    layout: Arc<Layout<St>>
}
impl<St: ChartState> RhubarbApp<St> {
    pub fn new(layout: Layout<St>) -> RhubarbApp<St> {
        RhubarbApp {
            layout: Arc::new(layout)
        }
    }
    pub fn start<Gen: GenerateUpdate<St> + 'static>(&self, gen: Gen) {
        let addr = "127.0.0.1:7878";
        println!("Listening for requests at http://{}", addr);
        gotham::start(addr, router::router(gen, self.layout.clone()))
    }
}






// pub struct Container<O> {
//     graph: Graph<O>,
//     controls: Option<Vec<Box<Control>>>,
//     // controls_order: Option<HashMap<u64, usize>>,
// }
// impl<O> Container<O> {
//     pub fn add_control<C: 'static + Control>(&mut self, control: C) {
//         match self.controls {
//             Some(ref mut controls) => {
//                 // debug_assert!(self.controls_order.is_some());
//                 // self.controls_order.as_mut().unwrap().insert(id, controls.len());
//                 controls.push(Box::new(control));
//             },
//             None => {
//                 self.controls = Some(vec![Box::new(control)]);
//                 // let mut order = HashMap::new();
//                 // order.insert(id, 0);
//                 // self.controls_order = Some(order);
//             }
//         }
//     }
// }
// impl<O: Serialize + Default> Serialize for Container<O> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
//         let mut state = serializer.serialize_struct("Container", 2)?;
//         state.serialize_field("plot", &self.graph)?;
//         println!("SERIALIZING CONTROLS {}", self.controls.as_ref().unwrap().len());
//         state.serialize_field("controls", &self.controls)?;
//         // state.serialize_field("state", &self.state)?;
//         state.end()
//     }
// }
// impl<O> From<Graph<O>> for Container<O> {
//     fn from(graph: Graph<O>) -> Container<O> {
//         // Container { graph, controls: None, controls_order: None }
//         Container { graph, controls: None }
//     }
// }

pub trait GraphKind: serde::Serialize + Default {}
impl<T> GraphKind for T where T: serde::Serialize + Default {}

// pub trait GenerateContainer<St>: Send + Sync + RefUnwindSafe + Clone {
//     type Output: GraphKind;
//     fn generate(&self, state: St) -> error::Result<Container<Self::Output>>;
// }

// /// Implement GenerateContainer for functions. Due to the `Clone` requirement, this will not work for
// /// closures currently, but will in the future (for non-mutating closures): see
// /// [this RFC](https://github.com/rust-lang/rfcs/pull/2132)
// /// and [this tracking issue](https://github.com/rust-lang/rust/issues/44490).
// impl<St, F, O> GenerateContainer<St> for F
//     where F: Fn(St) -> error::Result<Container<O>> + Send + Sync + RefUnwindSafe + Clone,
//           O: GraphKind
// {
//     type Output = O;

//     fn generate(&self, state: St) -> error::Result<Container<O>> {
//         self(state)
//     }
// }

// pub fn start<St, Gen: GenerateContainer<St> + 'static>(gen: Gen)
//     where St: 'static + Send + Sync + RefUnwindSafe + Serialize + DeserializeOwned + Default
// {
//     let addr = "127.0.0.1:7878";
//     println!("Listening for requests at http://{}", addr);
//     gotham::start(addr, router::router(gen))
// }
