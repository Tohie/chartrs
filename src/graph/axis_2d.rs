use graph::data_set::DataSet;
use graph::PointStyle;
use canvas::Canvas;
use pixel::{Pixel, GraphCoord, Color};

/// The Axis2D represents a 2D axis for a line or bar graph
pub struct Axis2D<'a, T: 'a> {
    /// max_x is the largest value that will be plotted on the x axis
    max_x: f64,
    /// max_y is the largest value that will be plotted on the y axis
    max_y: f64,

    min_x: f64,
    min_y: f64,

    /// The width of the canvas that Axis2D should be plotted on
    width: f64,
    /// The height of the canvas that Axis2D should be plotted on
    height: f64,

    /// vertical_border should be a number between 0 and 1 that represents
    /// the percentage of the vertical space available that should be used as 
    /// a margin between the canvas and the top and bottom of the y-axis
    vertical_border: f64,
    horizontal_border: f64,

    /// The canvas that the axis should be plotted on
    canvas: &'a mut T
}

impl<'c, T: Canvas> Axis2D<'c, T> {
    pub fn new(max_x: f64, max_y: f64, min_x: f64, min_y: f64, canvas: &'c mut T) -> Axis2D<'c, T> {
        let (w, h) = canvas.get_size();

        Axis2D {
            max_x: max_x,
            max_y: max_y,

            min_x: min_x,
            min_y: min_y,

            width: w,
            height: h,

            vertical_border: 0.1,
            horizontal_border: 0.1,

            canvas: canvas,
        }
    }

    pub fn set_horizontal_border(&mut self, horizontal_border: f64) {
        self.horizontal_border = horizontal_border;
    }

    pub fn set_vertical_border(&mut self, vertical_border: f64) {
        self.vertical_border = vertical_border;
    }

    pub fn show(&mut self) {
        self.canvas.show();
    }

    pub fn set_color<C: Into<Color>>(&mut self, c: C) {
        self.canvas.set_color(c)
    }

    /// This functions takes a GraphCoord and returns
    /// the pixel where it should be drawn on screen i.e. (2, 2)
    /// might be at the pixel (100, 100) on a particular canvas 
    fn graph_coord_to_pixel<G: Into<GraphCoord>>(&self, gp: G) -> Pixel {
        let gp = gp.into();
        let origin = self.canvas.get_origin();

        let x_range = self.max_x - self.min_x;
        let y_range = self.max_y - self.min_y;

        let x_origin_pixel = origin.x + (self.width * self.horizontal_border);
        let actual_width_pixels = self.width - (2.0 * self.width * self.horizontal_border);
        let new_x = x_origin_pixel + (actual_width_pixels * ((self.min_x.abs() + gp.x) / x_range));
        
        let y_origin_pixel = origin.y + (self.height * self.vertical_border);
        let actual_height_pixels = self.height - (2.0 * self.height * self.vertical_border);
        let new_y = y_origin_pixel + (actual_height_pixels * ((self.min_y.abs() + gp.y) / y_range));

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

    pub fn plot_axises(&mut self, tick_count: f64) {
        let max_x = self.max_x; let min_x = self.min_x;
        let max_y = self.max_y; let min_y = self.min_y;

        let (max_x, min_x, tick_x) = self.calculate_pretty_axis_values(max_x, min_x, tick_count);
        self.max_x = max_x; self.min_x = min_x;

        let mut x = min_x;
        while x <= max_x {
            let pix = self.graph_coord_to_pixel((x, 0.0));
            self.canvas.draw_line((pix.x, pix.y), (pix.x, pix.y-5.0));

            self.canvas.write_num(x, (pix.x - 3.0, pix.y - 10.0));

            x += tick_x;
        }

        let (max_y, min_y, tick_y) = self.calculate_pretty_axis_values(max_y, min_y, tick_count);
        self.max_y = max_y; self.min_y = min_y;

        let mut y = min_y;
        while y <= max_y {
            let pix = self.graph_coord_to_pixel((0.0, y));
            self.canvas.draw_line((pix.x, pix.y), (pix.x-5.0, pix.y));        

            self.canvas.write_num(y, (pix.x - 20.0, pix.y + 5.0));
            
            y += tick_y;
        }

        self.plot_line((min_x, 0.0), (max_x, 0.0));
        self.plot_line((0.0, min_y), (0.0, max_y));
        self.canvas.show();
    }

    // returns the upper and lower limits and the increment size
    fn calculate_pretty_axis_values(&self, max: f64, min: f64, tick_count: f64) -> (f64, f64, f64) {
        let range = max - min;
        let temp_step = range/(tick_count - 1.0);

        let mag = (temp_step.log10() - 1.0).floor();
        let ten = 10.0_f64;
        let mag_pow = ten.powf(mag);
       
        let step_size = (temp_step / mag_pow).ceil() * mag_pow;
        let ll = step_size * (min/step_size).floor();
        let ul = step_size * (max/step_size).ceil();

        (ul, ll, step_size)
    }

    pub fn write_xlabel(&mut self, x_label: &str) {
        let half_way = self.max_x / 2.0;
        let pix = self.graph_coord_to_pixel((half_way, 0.0));
        self.canvas.write_text(x_label, (pix.x, pix.y - 25.0));
    }

    pub fn write_ylabel(&mut self, y_label: &str) {
        let half_way = self.max_y / 2.0;
        let pix = self.graph_coord_to_pixel((0.0, half_way));
        self.canvas.write_text(y_label, (pix.x - 55.0, pix.y));
    }
}