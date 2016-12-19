mod graph_2d;
mod data_set;
mod axis_2d;

pub use self::data_set::DataSet;
pub use self::graph_2d::Graph2D;
pub use self::axis_2d::Axis2D;

#[derive(Clone, Copy, PartialEq)]
pub enum PlotStyle {
    Bar,
    Line,
    Scatter,  
}

#[derive(Clone, Copy, PartialEq)]
pub enum PointStyle {
    Nothing,
    Cross,
    // could have circle, etc.
}

pub trait Graph {
    fn plot(&mut self);
}
