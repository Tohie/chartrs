use graph::{PointStyle, AxisOptions};
use canvas::Canvas;
use pixel::{Pixel, GraphCoord, Color};

/// The Axis2D represents a 2D axis for a line or bar graph
/// This should be constructed for you automatically by a Graph2D
pub struct Axis2D<'a, 'b, T: 'a> {
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

    /// The amount in pixels each tick on the axis should be seperated by
    tick_amount_x: f64,
    tick_amount_y: f64,

    options: AxisOptions<'b>,

    /// The canvas that the axis should be plotted on
    canvas: &'a mut T
}

impl<'a, 'b, T: Canvas> Axis2D<'a, 'b, T> {
    pub fn new(max_x: f64, max_y: f64, min_x: f64, min_y: f64, opts: AxisOptions<'b>, canvas: &'a mut T) -> Axis2D<'a, 'b, T> {
        let (w, h) = canvas.get_size();
        
        let (max_x, min_x, tick_x) = calculate_pretty_axis_values(max_x, min_x, opts.tick_count);
        let (max_y, min_y, tick_y) = calculate_pretty_axis_values(max_y, min_y, opts.tick_count);
        
        Axis2D {
            max_x: max_x,
            max_y: max_y,

            min_x: min_x,
            min_y: min_y,

            width: w,
            height: h,

            tick_amount_x: tick_x,
            tick_amount_y: tick_y,

            options: opts,

            canvas: canvas,
        }
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
        let horizontal_border = self.options.horizontal_border;
        let vertical_border = self.options.vertical_border;

        let x_range = self.max_x - self.min_x;
        let y_range = self.max_y - self.min_y;

        let x_origin_pixel = origin.x + (self.width * horizontal_border);
        let actual_width_pixels = self.width - (2.0 * self.width * horizontal_border);
        let new_x = x_origin_pixel + (actual_width_pixels * ((self.min_x.abs() + gp.x) / x_range));
        
        let y_origin_pixel = origin.y + (self.height * vertical_border);
        let actual_height_pixels = self.height - (2.0 * self.height * vertical_border);
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

        let offset = self.width * 0.005;
        
        self.canvas.draw_line((x-offset, y), (x+offset, y));
        self.canvas.draw_line((x, y-offset), (x, y+offset));
    }

    pub fn plot_axises(&mut self) {
        // These need to be here, as self can't be borrowed immutably
        // during the mutable borrow of draw_axis();
        let max_x = self.max_x; let min_x = self.min_x;
        let max_y = self.max_y; let min_y = self.min_y;
        let height = self.height; let width = self.width;
        let tick_x = self.tick_amount_x; let tick_y = self.tick_amount_y;

        let is_x = true; let is_y = false;
        self.draw_axis(is_x, is_y, min_x, max_x, height, tick_x);

        let is_x = false; let is_y = true;
        self.draw_axis(is_x, is_y, min_y, max_y, width, tick_y);

        self.plot_line((min_x, 0.0), (max_x, 0.0));
        self.plot_line((0.0, min_y), (0.0, max_y));
        self.write_xlabel(self.options.x_label);
        self.write_ylabel(self.options.y_label);
        self.canvas.show();
    }

    pub fn draw_axis(&mut self, is_x: bool, is_y: bool, min_i: f64, max_i: f64, space: f64, tick: f64) {
        let is_x = if is_x { 1.0 } else { 0.0 };
        let is_y = if is_y { 1.0 } else { 0.0 };
        let mut i = min_i;
        while i <= max_i {
            let pix = self.graph_coord_to_pixel((i*is_x, i*is_y));
            
            let tick_size = space * self.options.tick_size;
            self.canvas.draw_line((pix.x, pix.y), (pix.x - (tick_size*is_y), pix.y - (tick_size*is_x)));

            let number_offset = space * self.options.number_offset;
            self.canvas.write_num(i, (pix.x - (number_offset * is_y), pix.y - (number_offset * is_x)));

            i += tick;
        }
    }

    fn write_label<P: Into<Pixel>>(&mut self, label: &str, loc: P) {
        let pix = self.graph_coord_to_pixel(loc);
        let x_offset = self.width * self.options.label_offset;
        let y_offset = self.height * self.options.label_offset;
        self.canvas.write_text(label, (pix.x - x_offset, pix.y - y_offset));
    }

    fn write_xlabel(&mut self, x_label: &str) {
        let half_way = self.max_x  / 2.0;
        self.write_label(x_label, (half_way, 0.0));
    }

    fn write_ylabel(&mut self, y_label: &str) {
        let half_way = self.max_y / 2.0;
        self.write_label(y_label, (0.0, half_way));
    }
}

// returns the upper and lower limits and the increment size
fn calculate_pretty_axis_values(max: f64, min: f64, tick_count: f64) -> (f64, f64, f64) {
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