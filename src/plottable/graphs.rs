use data_set::DataSet;
use plottable::primitives::*;
use plottable::{Plottable, HasDataSet};
use graph_dimensions::GraphDimensions;
use canvas::Canvas;

pub struct LineSeries<'a>(pub &'a DataSet<'a>);

impl <'a> Plottable for LineSeries<'a> {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let ds = self.0;
        canvas.set_color(ds.choose_color());
        
        for pair in ds.data_points.windows(2) {
            Line(pair[0], pair[1]).plot(bounds, canvas);
        }
    }
}

impl <'a> HasDataSet for LineSeries<'a> {
    fn data_set(&self) -> &DataSet {
        self.0
    }
}

pub struct ScatterSeries<'a>(pub &'a DataSet<'a>);

impl <'a> Plottable for ScatterSeries<'a> {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let ds = self.0;

        for &point in ds.data_points.iter() {
            canvas.set_color(ds.choose_color());
            Point(point, ds.options.point_style).plot(bounds, canvas);
        } 
    }
}

impl <'a> HasDataSet for ScatterSeries<'a> {
    fn data_set(&self) -> &DataSet {
        self.0
    }
}

pub struct BarSeries<'a>(pub &'a DataSet<'a>);

impl <'a> Plottable for BarSeries<'a> {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let ds = self.0;

        for &point in ds.data_points.iter() {
            canvas.set_color(ds.choose_color());
            Bar(point).plot(bounds, canvas);
        }
    }
}