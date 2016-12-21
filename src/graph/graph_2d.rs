use graph::data_set::DataSet;
use graph::{Graph, PlotStyle, AxisOptions, DataSetOptions};
use graph::axis_2d::Axis2D;
use pixel::{GraphCoord, Color};
use canvas::Canvas;
use utils;

/// A `Graph2D` is a graph with a standard 2d axis, i.e. a bar, line or a scatter graph
pub struct Graph2D<'a: 'c, 'b, 'c, 'd, T: 'b> {
    data_sets: &'c [&'a DataSet<'a>],
    axis: Axis2D<'b, 'd, T>,
}

impl <'a, 'b, 'c, 'd, T: Canvas> Graph2D<'a, 'b, 'c, 'd, T> {
    pub fn new(canvas: &'b mut T, data_sets: &'c [&'a DataSet<'a>], axis_options: AxisOptions<'d>) -> Self {
        let max_coords = data_sets.iter()
            .map(|ds| ds.get_max_coord())
            .collect::<Vec<GraphCoord>>();

        let max_coord = utils::get_max_coord(&max_coords);

        let min_coords = data_sets.iter()
            .map(|ds| ds.get_min_coord())
            .collect::<Vec<GraphCoord>>();

        let min_coord = utils::get_min_coord(&min_coords);

        let mut axis = Axis2D::new(max_coord.x, max_coord.y, min_coord.x, min_coord.y, axis_options, canvas);
        Graph2D { data_sets: data_sets, axis: axis, }
    }

    fn plot_line_graph(&mut self, ds: &'a DataSet<'a>) {
        for pair in ds.data_points.windows(2) {
            self.update_color(ds);
            self.axis.plot_line(pair[0], pair[1]);
        }
    }

    fn plot_point_as_bar(&mut self, gp: GraphCoord) {
        let x = gp.x;
        let y = gp.y;

        self.axis.plot_line((x - 0.5, 0.0), (x - 0.5, y));
        self.axis.plot_line((x - 0.5, y), (x + 0.5, y));
        self.axis.plot_line((x + 0.5, y), (x + 0.5, 0.0));
        self.axis.plot_line((x - 0.5, 0.0), (x + 0.5, 0.0));
    }

    fn plot_bar_graph(&mut self, ds: &'a DataSet<'a>) {
        for &point in ds.data_points.iter() {
            self.update_color(ds);
            self.plot_point_as_bar(point);
        }
    }

    fn plot_scatter_graph(&mut self, ds: &'a DataSet<'a>) {
        for &point in ds.data_points.iter() {
            self.update_color(ds);
            self.axis.plot_point(point, ds.options.point_style);
        }
    }

    /// Convenience method that creates an axis and a data_set
    /// from the arguments provided and plots it
    pub fn plot_fn<F>(c: &'a mut T, data_set_opts: &'a DataSetOptions<'a>, plot_options: AxisOptions<'b>, xs: Vec<f64>, f: F) 
        where F: Fn(f64) -> f64 {

        let ds = DataSet::from_fn(xs, data_set_opts, f);
        let data_sets = &[&ds];

        let mut graph = Graph2D::new(c, data_sets, plot_options);
        graph.plot();
    }

    pub fn update_color(&mut self, ds: &'a DataSet<'a>) {
        self.axis.set_color(ds.choose_color());
    }

    pub fn plot_data_set(&mut self, ds: &'a DataSet) {
        match ds.options.plot_style {
            PlotStyle::Line => self.plot_line_graph(ds),
            PlotStyle::Bar => self.plot_bar_graph(ds),
            PlotStyle::Scatter => self.plot_scatter_graph(ds),
        }

        self.axis.show();
    }
}

impl <'a, 'b, 'c, 'd, T: Canvas> Graph for Graph2D<'a, 'b, 'c, 'd, T> {
    fn plot(&mut self) {
        self.axis.set_color(Color(0, 0, 0));
        self.axis.plot_axises();

        for ds in self.data_sets {
            self.plot_data_set(ds);
        } 
    }
}