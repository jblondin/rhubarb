#[macro_use] extern crate serde_derive;
#[macro_use] extern crate derive_graph_elem;
extern crate agnes;
extern crate serde;
extern crate palette;
extern crate num_traits;

pub mod scatter;

mod layout;
pub use layout::{Layout};

mod graph;
pub use graph::Graph;

mod traits;
pub use traits::{SingleOrMore, CountExistFields};

pub mod color;
pub mod common;
