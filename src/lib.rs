//! The `chartrs` crate is used for creating and drawing graphs in rust
//! The canvas module containts the `Canvas` trait
//! anything implementing the `Canvas` trait can be used to draw
//! a graph using this crate
//! The graph module contains a `DataSet` struct which holds
//! the data for a graph as well as any options such as what colour to draw the graph
//! The `Graph2D` is the main struct in this crate, it holds several DataSets
//! and an axis to plot a graph.
//!
//! An example usage of how this ties together:
//! 
//! # Examples
//!
//! ```rust
//! # extern crate sdl2;
//! use chartrs::canvas::with_sdl2_context;
//! use chartrs::{Graph2D, DataSetOptions};
//!
//! fn main() {
//!     let font_size = 12;
//!     with_sdl2_context(800, 600, font_size, |ctx| {
//!         // See the DataSetOptions and PlotOptions structs for the available options
//!         // The default here is to plot a line graph in black
//!         Graph2D::plot_fn(ctx, DataSetOptions::default(), PlotOptions::default(), 
//!             vec!(-1.0, 0, 1.0, 2.0, 3.0, 4.0), |x| x.powi(2));
//!     }
//! }
//! ```

extern crate sdl2;
extern crate rand;

pub mod graph;
pub mod pixel;
pub mod canvas;
mod utils;

pub use pixel::Pixel;
pub use canvas::Canvas;
pub use graph::{Graph2D, DataSet, PlotStyle, PointStyle, AxisOptions, DataSetOptions};
