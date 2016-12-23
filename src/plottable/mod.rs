mod axis;
pub mod primitives;
pub mod graphs;

pub use self::axis::{Axis, AxisKind};

use canvas::Canvas;
use graph_bounds::GraphBounds;
use data_set::DataSet;

pub trait Plottable {
    fn plot<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C);
}

/// Series is an extension to Plottable,
/// it is meant for anything that can be plotted
/// as a series on a graph such a line series
pub trait Series : Plottable {
    fn data_set(&self) -> &DataSet;
}