pub mod axis;
pub mod primitives;
pub mod graphs;
pub mod legend;

pub use self::axis::Axis;
pub use self::legend::Legend;

use canvas::Canvas;
use graph_dimensions::GraphDimensions;
use data_set::DataSet;

pub trait Plottable {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) -> Result<(), C::Err>;
}

pub trait HasDataSet {
    fn data_set(&self) -> &DataSet;
}