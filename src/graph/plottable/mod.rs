mod axis;
pub mod primitives;

pub use self::axis::{Axis, AxisKind};

use canvas::Canvas;
use graph::canvas::GraphBounds;

pub trait Plottable {
    fn plot<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C);
}