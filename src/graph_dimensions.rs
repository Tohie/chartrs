use pixel::{GraphCoord, Pixel};
use utils;
use data_set::DataSet;
use options::AxisOptions;
use plottable::axis::{AxisKind, Axis};
use std::f64;

#[derive(Clone, Copy, PartialEq)]
pub struct GraphDimensions {
    pub max: GraphCoord,
    pub min: GraphCoord,
 
    pub height: f64,
    pub width: f64,

    pub horizontal_border: f64,
    pub vertical_border: f64,
}

impl GraphDimensions {
    pub fn new(width: f64, height: f64) -> GraphDimensions {
        GraphDimensions {
            max: GraphCoord::new(f64::MIN, f64::MIN),
            min: GraphCoord::new(f64::MAX, f64::MAX),

            height: height,
            width: width,

            horizontal_border: 0.1,
            vertical_border: 0.1,
        }
    }

    pub fn convert_to_pixel<G: Into<GraphCoord>>(&self, gp: G) -> Pixel {
        let mut gp = gp.into();

        if gp.x < self.min.x { gp.x = self.min.x };
        if gp.y < self.min.y { gp.y = self.min.y };
        if gp.x > self.max.x { gp.x = self.max.x };
        if gp.y > self.max.y { gp.y = self.max.y };

        let origin = Pixel::new(0.0, 0.0);
        let horizontal_border = self.horizontal_border;
        let vertical_border = self.vertical_border;

        let x_range = self.max.x - self.min.x;
        let y_range = self.max.y - self.min.y;

        let x_origin_pixel = origin.x + (self.width * horizontal_border);
        let actual_width_pixels = self.width - (2.0 * self.width * horizontal_border);
        let new_x = x_origin_pixel + (actual_width_pixels * ((gp.x - self.min.x) / x_range));
        
        let y_origin_pixel = origin.y + (self.height * vertical_border);
        let actual_height_pixels = self.height - (2.0 * self.height * vertical_border);
        let new_y = y_origin_pixel + (actual_height_pixels * ((gp.y - self.min.y) / y_range));

        Pixel::new(new_x, new_y)
    }

    pub fn adjust_for(&mut self, ds: &DataSet) {
        let max = utils::get_max_coord(&[self.max, ds.get_max_coord()]);
        let min = utils::get_min_coord(&[self.min, ds.get_min_coord()]);

        self.max = max;
        self.min = min;
    }

    // 
    pub fn adjust_for_axis(&mut self, x_num_ticks: f64, y_num_ticks: f64) {
        let (max_x, min_x, tick_x) = utils::pretty_axis_values(self.max.x, self.min.x, x_num_ticks);
        let (max_y, min_y, tick_y) = utils::pretty_axis_values(self.max.y, self.min.y, y_num_ticks);

        self.max = GraphCoord::new(max_x, max_y);
        self.min = GraphCoord::new(min_x, min_y);
    }
}