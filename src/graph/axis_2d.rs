use graph::data_set::DataSet;
use graph::PointStyle;
use canvas::Canvas;
use pixel::{Pixel, GraphCoord};

/// The Axis2D represents a 2D axis for a line or bar graph
/// The main purpose of the Axis2D is to translate graph coordinates
/// to actual pixels so that the underlying canvas can draw them in the correct 
/// place.
/// It also calculates what the increment of the x and y ticks should be as well
/// as the drawing the axises correctly obviously
pub struct Axis2D<'a, T: 'a> {
    /// max_x_value is the largest value that will be plotted on the x axis
    max_x_value: f64,
    /// max_y_value is the largest value that will be plotted on the y axis
    max_y_value: f64,

    min_x_value: f64,
    min_y_value: f64,

    /// The width of the canvas that Axis2D should be plotted on
    width: f64,
    /// The height of the canvas that Axis2D should be plotted on
    height: f64,

    /// The amount that each "tick" on the x axis increases by
    increment_x: f64,
    /// The amount that each "tick" on the y axis increases by
    increment_y: f64,

    /// vertical_border should be a number between 0 and 1 that represents
    /// the percentage of the vertical space available that should be used as 
    /// a margin between the canvas and the top and bottom of the y-axis
    vertical_border: f64,
    horizontal_border: f64,

    /// The canvas that the axis should be plotted on
    canvas: &'a mut T
}

impl<'c, T: Canvas> Axis2D<'c, T> {
    /// Returns a new Axis2D with default values that should be changed before use
    /// 
    /// # Arguments
    /// * `ds` - A DataSet that contains the points to be plotted on the axis
    pub fn new(ds: &'c DataSet, canvas: &'c mut T) -> Axis2D<'c, T> {
        let (max_x_value, max_y_value) = ds.get_max_x_and_y();
        let (min_x_value, min_y_value) = ds.get_min_x_and_y();

        let (w, h) = canvas.get_size();

        let mut axis = Axis2D {
            max_x_value: max_x_value,
            max_y_value: max_y_value,

            min_x_value: min_x_value,
            min_y_value: min_y_value,

            increment_x: 1.0,
            increment_y: 1.0,

            width: w,
            height: h,

            vertical_border: 0.1,
            horizontal_border: 0.1,

            canvas: canvas,
        };

        axis.plot_axises();
        axis
    }

    /// This functions takes a GraphCoord and returns
    /// the pixel where it should be drawn on screen i.e. (2, 2)
    /// might be at the pixel (100, 100) on a particular canvas 
    fn graph_coord_to_pixel(&self, gp: GraphCoord) -> Pixel {
        let origin = self.canvas.get_origin();

        let x_range = self.max_x_value - self.min_x_value;
        let y_range = self.max_y_value - self.min_y_value;

        let x_origin_pixel = origin.x + (self.width * self.horizontal_border);
        let actual_width_pixels = self.width - (2.0 * self.width * self.horizontal_border);
        let new_x = x_origin_pixel + (actual_width_pixels * ((self.min_x_value.abs() + gp.x) / x_range));
        
        let y_origin_pixel = origin.y + (self.height * self.vertical_border);
        let actual_height_pixels = self.height - (2.0 * self.height * self.vertical_border);
        let new_y = y_origin_pixel + (actual_height_pixels * ((self.min_y_value.abs() + gp.y) / y_range));

        Pixel::new(new_x, new_y)
    }

    pub fn plot_line<G: Into<GraphCoord>>(&mut self, p1: G, p2: G) {
        let p1 = self.graph_coord_to_pixel(p1.into());
        let p2 = self.graph_coord_to_pixel(p2.into());
        self.canvas.draw_line(p1, p2);
    }

    pub fn plot_point<G: Into<GraphCoord>>(&mut self, point: G, point_style: PointStyle) {
        match point_style {
            PointStyle::Cross => self.plot_cross(point.into()),
            _ => {},
        }
    }

    fn plot_cross(&mut self, point: GraphCoord) {
        let pix = self.graph_coord_to_pixel(point);
        let x = pix.x;
        let y = pix.y;

        self.canvas.draw_line((x-5.0, y), (x+5.0, y));
        self.canvas.draw_line((x, y-5.0), (x, y+5.0));
    }

    pub fn show(&mut self) {
        self.canvas.show();
    }

    fn plot_axises(&mut self) {
        let max_x = self.max_x_value; let min_x = self.min_x_value;
        let max_y = self.max_y_value; let min_y = self.min_y_value;

        let mut x = self.min_x_value;
        while x <= self.max_x_value {
            let pix = self.graph_coord_to_pixel(GraphCoord::new(x, 0.0));
            self.canvas.draw_line((pix.x, pix.y), (pix.x, pix.y-5.0));

            let num = format!("{}", x);
            self.canvas.write_text(&num, (pix.x - 3.0, pix.y - 10.0));

            x += self.increment_x;
        }

        let mut y = self.min_y_value;
        while y <= self.max_y_value {
            let pix = self.graph_coord_to_pixel(GraphCoord::new(0.0, y));
            self.canvas.draw_line((pix.x, pix.y), (pix.x-5.0, pix.y));

            let num = format!("{}", y);
            self.canvas.write_text(&num, (pix.x - 20.0, pix.y + 5.0));
            
            y += self.increment_y;
        }
        self.plot_line((min_x, 0.0), (max_x, 0.0));
        self.plot_line((0.0, min_y), (0.0, max_y));
        self.canvas.show();
    }
}