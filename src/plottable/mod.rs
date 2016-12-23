pub mod axis;
pub mod primitives;
pub mod graphs;

pub use self::axis::{Axis, AxisKind};

use canvas::Canvas;
use graph_dimensions::GraphDimensions;
use data_set::DataSet;

pub trait Plottable {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C);
}

pub trait HasDataSet {
    fn data_set(&self) -> &DataSet;
}