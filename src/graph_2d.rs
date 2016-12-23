use data_set::DataSet;
use options::{PlotStyle, AxisOptions};
use plottable::{Plottable, HasDataSet};
use plottable::primitives::Bar;
use plottable::graphs::{LineSeries, ScatterSeries, BarSeries};
use plottable::axis::Axis;
use pixel::{GraphCoord, Color};
use graph_dimensions::GraphDimensions;
use canvas::Canvas;

/// A `Graph2D` is a graph with a standard 2d canvas, i.e. a bar, line or a scatter graph
pub struct Graph2D<'a, 'c, T: 'c> {
    data_sets: Vec<&'a DataSet<'a>>,
    canvas: &'c mut T,
    dimensions: GraphDimensions,
}

impl <'a, 'c, T: Canvas> Graph2D<'a, 'c, T> {
    pub fn new(canvas: &'c mut T, data_sets: Vec<&'a DataSet<'a>>) -> Self {
        let (width, height) = canvas.get_size();
        let mut dimensions = GraphDimensions::new(width, height);

        for ds in data_sets.iter() {
            dimensions.adjust_for(ds);
        }

        Graph2D { 
            data_sets: data_sets, 
            canvas: canvas,
            dimensions: dimensions,
        }
    }

    pub fn add_data_set(&mut self, data_set: &'a DataSet<'a>) {
        self.dimensions.adjust_for(data_set);
        self.data_sets.push(data_set);
    }

    pub fn plot_data_set(&mut self, ds: &'a DataSet) {
        match ds.options.plot_style {
            PlotStyle::Line => self.plot(&LineSeries(ds)),
            PlotStyle::Bar => self.plot(&BarSeries(ds)),
            PlotStyle::Scatter => self.plot(&ScatterSeries(ds)),
        }

        self.canvas.show();
    }

    fn plot<P: Plottable>(&mut self, p: &P) {
        p.plot(&self.dimensions, self.canvas)
    }

    pub fn show(&mut self, x_opts: &AxisOptions, y_opts: &AxisOptions) {
        self.canvas.set_color(Color(255, 255, 255));
        self.canvas.clear();
        
        self.dimensions.adjust_for_axis(x_opts.tick_count, y_opts.tick_count);
        let axis = Axis::new(x_opts, y_opts);
            
        self.plot(&axis);
        
        let data_sets = self.data_sets.clone();
        for ds in data_sets.iter() {
            self.plot_data_set(ds);
        } 
    }
}