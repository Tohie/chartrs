use data_set::DataSet;
use options::{PlotStyle, AxisOptions};
use plottable::{Plottable, Axis, Legend};
use plottable::graphs::{LineSeries, ScatterSeries, BarSeries};
use pixel::Color;
use graph_dimensions::GraphDimensions;
use canvas::Canvas;

/// A `Graph2D` is a graph with a standard 2d canvas, i.e. a bar, line or a scatter graph
pub struct Graph2D<'a, 'c, 'o, T: 'c> {
    pub data_sets: Vec<&'a DataSet<'a>>,
    canvas: &'c mut T,
    pub dimensions: GraphDimensions,
    x_opts: Option<&'o AxisOptions<'o>>,
    y_opts: Option<&'o AxisOptions<'o>>,
}

impl <'a, 'c, 'o, T: Canvas> Graph2D<'a, 'c, 'o, T> {
    pub fn new(canvas: &'c mut T, data_sets: Vec<&'a DataSet<'a>>) -> Self {
        Graph2D::with_axises(canvas, data_sets, None, None)
    }

    pub fn with_axises<A>(canvas: &'c mut T, data_sets: Vec<&'a DataSet<'a>>, x_opts: A, y_opts: A) -> Self
        where A: Into<Option<&'o AxisOptions<'o>>> {

        let dimensions = GraphDimensions::from(canvas, &data_sets);

        Graph2D {
            data_sets: data_sets,
            canvas: canvas,
            dimensions: dimensions,
            x_opts: x_opts.into(),
            y_opts: y_opts.into(),
        }
    }

    pub fn add_data_set(&mut self, data_set: &'a DataSet<'a>) {
        self.dimensions.adjust_for(data_set);
        self.data_sets.push(data_set);
    }

    pub fn plot_data_set(&mut self, ds: &'a DataSet) -> Result<(), T::Err> {
        match ds.options.plot_style {
            PlotStyle::Line => self.plot(&LineSeries(ds)),
            PlotStyle::Bar => self.plot(&BarSeries(ds)),
            PlotStyle::Scatter => self.plot(&ScatterSeries(ds)),
        }
    }

    fn plot<P: Plottable>(&mut self, p: &P) -> Result<(), T::Err> {
        p.plot(&self.dimensions, self.canvas)
    }

    pub fn fit_view_to_data(&mut self) {
        self.dimensions = GraphDimensions::from(self.canvas, &self.data_sets);
    }

    fn redraw_data_sets(&mut self, prettify_axises: bool) -> Result<(), T::Err> {
        self.canvas.set_color(Color(255, 255, 255));
        self.canvas.clear();
        
        // We create a new axis each time show is called because axis can't be stored on Graph2D and plotted
        // without cloning it anyway because you would have borrow self mutably
        // to plot axis and borrow self.axis at the same
        match (self.x_opts, self.y_opts) {
            (Some(x_opts), Some(y_opts)) => {
                let axis = Axis::from_dimensions(&self.dimensions, x_opts, y_opts);
                if prettify_axises {
                    self.dimensions.adjust_for_axis(&axis);
                }    
                self.plot(&axis)?;
            }
            _ => {},
        };

        let data_sets = self.data_sets.clone();
        for ds in data_sets.iter() {
            self.plot_data_set(ds)?;
        } 

        self.plot(&Legend(&data_sets))?;

        self.canvas.show();
        Ok(())
    }

    pub fn show(&mut self) -> Result<(), T::Err> {
        self.redraw_data_sets(true)
    }

    pub fn move_view(&mut self, x: f64, y: f64) -> Result<(), T::Err> {
        self.dimensions.max.x += x;
        self.dimensions.min.x += x;

        self.dimensions.max.y += y;
        self.dimensions.min.y += y;

        self.redraw_data_sets(false)
    }

    pub fn scale_horizontal(&mut self, x: f64) -> Result<(), T::Err> {
        self.dimensions.max.x *= x;
        self.dimensions.min.x *= x;

        self.redraw_data_sets(false)
    }

    pub fn scale_vertical(&mut self, y: f64) -> Result<(), T::Err> {
        self.dimensions.max.y *= y;
        self.dimensions.min.y *= y;
        
        self.redraw_data_sets(false)
    }

    pub fn scale(&mut self, factor: f64) -> Result<(), T::Err> {
        self.scale_horizontal(factor)?;
        self.scale_vertical(factor)
    }
}