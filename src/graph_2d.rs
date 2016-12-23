use data_set::DataSet;
use options::{PlotStyle, AxisOptions};
use plottable::Plottable;
use plottable::primitives::{Point, Bar};
use plottable::graphs::LineSeries;
use plottable::{Axis, AxisKind};
use pixel::{GraphCoord, Color};
use graph_bounds::GraphBounds;
use canvas::Canvas;
use utils;

/// A `Graph2D` is a graph with a standard 2d canvas, i.e. a bar, line or a scatter graph
pub struct Graph2D<'a: 'c, 'b, 'c, 'd, T: 'b> {
    data_sets: &'c [&'a DataSet<'a>],
    canvas: &'b mut T,
    bounds: GraphBounds,
    x_axis: Axis<'d>,
    y_axis: Axis<'d>,
}

impl <'a, 'b, 'c, 'd, T: Canvas> Graph2D<'a, 'b, 'c, 'd, T> {
    pub fn new(canvas: &'b mut T, data_sets: &'c [&'a DataSet<'a>], x_opts: &'d AxisOptions<'d>, y_opts: &'d AxisOptions<'d>) -> Self {
        let max_coords = data_sets.iter()
            .map(|ds| ds.get_max_coord())
            .collect::<Vec<GraphCoord>>();

        let max = utils::get_max_coord(&max_coords);

        let min_coords = data_sets.iter()
            .map(|ds| ds.get_min_coord())
            .collect::<Vec<GraphCoord>>();

        let min = utils::get_min_coord(&min_coords);

        let (width, height) = canvas.get_size();

        let (max_x, min_x, tick_x) = utils::pretty_axis_values(max.x, min.x, x_opts.tick_count);
        let (max_y, min_y, tick_y) = utils::pretty_axis_values(max.y, min.y, y_opts.tick_count);

        let x_axis = Axis::new(AxisKind::X, tick_x, max_x, min_x, width, x_opts);
        let y_axis = Axis::new(AxisKind::Y, tick_y, max_y, min_y, height, y_opts);

        let bounds = GraphBounds::new((min_x, min_y), (max_x, max_y), width, height);

        Graph2D { 
            data_sets: data_sets, 
            canvas: canvas,
            bounds: bounds,
            x_axis: x_axis,
            y_axis: y_axis, 
        }
    }

    fn plot_line_graph(&mut self, ds: &'a DataSet<'a>) {
        self.plot(&LineSeries(ds))
    }

    fn plot_point_as_bar(&mut self, gp: GraphCoord) {
        let x = gp.x;
        let y = gp.y;

        self.plot(&Bar(GraphCoord::new(x, y)));
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
            self.plot(&Point(point, ds.options.point_style));
        }
    }

    pub fn update_color(&mut self, ds: &'a DataSet<'a>) {
        self.canvas.set_color(ds.choose_color());
    }

    pub fn plot_data_set(&mut self, ds: &'a DataSet) {
        match ds.options.plot_style {
            PlotStyle::Line => self.plot_line_graph(ds),
            PlotStyle::Bar => self.plot_bar_graph(ds),
            PlotStyle::Scatter => self.plot_scatter_graph(ds),
        }

        self.canvas.show();
    }

    fn plot<P: Plottable>(&mut self, p: &P) {
        p.plot(&self.bounds, self.canvas)
    }

    pub fn show(&mut self) {
        self.canvas.set_color(Color(255, 255, 255));
        self.canvas.clear();

        let x_axis = self.x_axis;
        let y_axis = self.y_axis;
            
        self.plot(&x_axis);
        self.plot(&y_axis);

        self.canvas.set_color(Color(0, 0, 0));
        
        for ds in self.data_sets {
            self.plot_data_set(ds);
        } 
    }
}