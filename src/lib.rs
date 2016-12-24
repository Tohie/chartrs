//! The `chartrs` crate is used for creating and drawing graphs in rust
//! The canvas module containts the `Canvas` trait
//! anything implementing the `Canvas` trait can be used to draw
//! a graph using this crate
//! The graph module contains a `DataSet` struct which holds
//! the data for a graph as well as any options such as what colour to draw the graph
//! The `Graph2D` is the main struct in this crate, it holds several DataSets
//! and an axis to plot a graph.

extern crate sdl2;
extern crate rand;

pub mod pixel;
pub mod canvas;
pub mod options;
mod graph_2d;
mod data_set;
pub mod plottable;
mod graph_dimensions;
mod utils;
mod wilkinsons;

pub use pixel::Pixel;
pub use canvas::Canvas;
pub use data_set::DataSet;
pub use graph_dimensions::GraphDimensions;
pub use graph_2d::Graph2D;
