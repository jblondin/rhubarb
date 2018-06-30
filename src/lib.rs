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

use std::sync::Arc;
use std::panic::RefUnwindSafe;

use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;

use rhubarb_graph as rg;

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
