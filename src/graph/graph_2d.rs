use graph::data_set::DataSet;
use graph::{Graph, PlotStyle, PointStyle};
use graph::axis_2d::Axis2D;
use pixel::GraphCoord;
use canvas::Canvas;

/// A Graph2D is a graph with a standard 2d axis, i.e. a bar, line or a scatter graph
pub struct Graph2D<'a, T: 'a> {
    data_set: &'a DataSet,
    axis: &'a mut Axis2D<'a, T>,
    plot_style: PlotStyle,
}

impl <'a, T: Canvas> Graph2D<'a, T> {
    pub fn new(data_set: &'a DataSet, axis: &'a mut Axis2D<'a, T>, style: PlotStyle) -> Self {
        Graph2D { data_set: data_set, axis: axis, plot_style: style, }
    }

    fn plot_line_graph(&mut self) {
        for pair in self.data_set.data_points.windows(2) {
            self.axis.plot_line(pair[0], pair[1]);
        }
    }

    fn plot_point_as_bar(&mut self, gp: GraphCoord) {
        let x = gp.x;
        let y = gp.y;

        self.axis.plot_line((x - 0.5, 0.0), (x - 0.5, y));
        self.axis.plot_line((x - 0.5, y), (x + 0.5, y));
        self.axis.plot_line((x + 0.5, y), (x + 0.5, 0.0));
    }

    fn plot_bar_graph(&mut self) {
        for &point in self.data_set.data_points.iter() {
            self.plot_point_as_bar(point);
        }
    }

    fn plot_scatter_graph(&mut self) {
        for &point in self.data_set.data_points.iter() {
            self.axis.plot_point(point, self.data_set.point_style);
        }
    }

    /// Convenience method that creates an axis and a data_set
    /// from the arguments provided and plots it
    pub fn plot_fn<F>(c: &'a mut T, plot_style: PlotStyle, xs: Vec<f64>, f: F) 
        where F: Fn(f64) -> f64 {

        let ds = DataSet::from_fn(xs, f);
        let mut axis = Axis2D::new(&ds, c);

        let mut graph = Graph2D::new(&ds, &mut axis, plot_style);
        graph.plot();
    }
}

impl <'a, T: Canvas> Graph for Graph2D<'a, T> {
    fn plot(&mut self) {
        match self.plot_style {
            PlotStyle::Line => self.plot_line_graph(),
            PlotStyle::Bar => self.plot_bar_graph(),
            PlotStyle::Scatter => self.plot_scatter_graph(),
        }

        self.axis.show();
    }
}