use pixel::{GraphCoord, Pixel};
use utils;
use data_set::DataSet;
use options::AxisOptions;
use plottable::axis::{AxisKind, Axis};
use std::f64;

#[derive(Clone, Copy, PartialEq, Debug)]
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

    // TODO: Handle out of bounds `GraphCoord`s better
    pub fn convert_to_pixel<G: Into<GraphCoord>>(&self, gp: G) -> Pixel {
        let mut gp = gp.into();

        if gp.x < self.min.x { gp.x = self.min.x };
        if gp.y < self.min.y { gp.y = self.min.y };
        if gp.x > self.max.x { gp.x = self.max.x };
        if gp.y > self.max.y { gp.y = self.max.y };

        // TODO: Do something if border are invalid
        // i.e. not between 0 and 1
        
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

    // TODO: Write a test for this after utils::pretty_axis_values
    pub fn adjust_for_axis(&mut self, x_num_ticks: f64, y_num_ticks: f64) {
        let (max_x, min_x, tick_x) = utils::pretty_axis_values(self.max.x, self.min.x, x_num_ticks);
        let (max_y, min_y, tick_y) = utils::pretty_axis_values(self.max.y, self.min.y, y_num_ticks);

        self.max = GraphCoord::new(max_x, max_y);
        self.min = GraphCoord::new(min_x, min_y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixel::{GraphCoord, Pixel};
    use data_set::DataSet;
    use options::DataSetOptions;

    #[test]
    fn test_convert_to_pixels() {
        let mut dim = GraphDimensions::new(600.0, 600.0);

        // with all 4 quadrants
        dim.max = GraphCoord::new(10.0, 10.0);
        dim.min = GraphCoord::new(-10.0, -10.0);

        assert_eq!(dim.convert_to_pixel((0.0, 0.0)), Pixel::new(300.0, 300.0));
        assert_eq!(dim.convert_to_pixel((-3.0, 5.0)), Pixel::new(228.0, 420.0));

        // with just positive x and y
        dim.min = GraphCoord::new(0.0, 0.0);
        assert_eq!(dim.convert_to_pixel((2.0, 1.0)), Pixel::new(156.0, 108.0));

        
        // Test horizontal_border and vertical_border settings
        dim.horizontal_border = 0.2;
        dim.vertical_border = 0.3;

        assert_eq!(dim.convert_to_pixel((2.0, 1.0)), Pixel::new(192.0, 204.0));

        // TODO: convert_to_pixel should probably return None on out of bounds
        // instead of clamping the value to the axis
        // assert_eq!(dim.convert_to_pixel((20.0, -3.0)), None)

        // this is meaningless, so we don't have to return anything useful
        // however it should not panic
        dim.max = dim.min;
        dim.convert_to_pixel((2.0, 1.0));

    }

    #[test]
    fn test_adjust_for() {
        let mut dim = GraphDimensions::new(600.0, 600.0);

        // When creating a GraphDimensions adjusting for anything should change everything
        let default_ops = DataSetOptions::default();
        let ds = DataSet::from_fn(vec!(10.0), &default_ops, |x| 10.0);
        dim.adjust_for(&ds);

        assert_eq!(dim.max, GraphCoord::new(10.0, 10.0));
        assert_eq!(dim.min, GraphCoord::new(10.0, 10.0));

        // Testing when max and min have been previously set
        // This should change the max x and min y and nothing else
        dim.max = GraphCoord::new(10.0, 10.0);
        dim.min = GraphCoord::new(-10.0, -5.0);

        let ds = DataSet::from_fn(vec!(20.0), &default_ops, |x| -15.0);
        dim.adjust_for(&ds);

        assert_eq!(dim.max, GraphCoord::new(20.0, 10.0));
        assert_eq!(dim.min, GraphCoord::new(-10.0, -15.0));

        // Adding a DataSet that doesn't have any values higher or lower than
        // the current extremes shouldn't change anything
        let ds = DataSet::from_fn(vec!(0.0), &default_ops, |x| 0.0);
        dim.adjust_for(&ds);

        assert_eq!(dim.max, GraphCoord::new(20.0, 10.0));
        assert_eq!(dim.min, GraphCoord::new(-10.0, -15.0)); 
    }
}