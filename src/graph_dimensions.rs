use pixel::{GraphCoord, Pixel};
use utils;
use data_set::DataSet;
use canvas::Canvas;
use plottable::Axis;
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

    pub fn from<C: Canvas>(canvas: &mut C, data_sets: &[&DataSet]) -> GraphDimensions {
        let (width, height) = canvas.get_size();
        let mut dimensions = GraphDimensions::new(width, height);

        for ds in data_sets.iter() {
            dimensions.adjust_for(ds);
        }

        dimensions
    }
    
    // The width of the horizontal_border in pixels
    // instead of percent
    fn horizontal_border(&self) -> f64 {
        self.horizontal_border * self.width
    }

    fn vertical_border(&self) -> f64 {
        self.vertical_border * self.height
    }

    // The width of the grid without the horizontal_border either side
    fn actual_width(&self) -> f64 {
        self.width - (2.0 * self.horizontal_border())
    }

    fn actual_height(&self) -> f64 {
        self.height - (2.0 * self.vertical_border())
    }

    fn x_range(&self) -> f64 {
        self.max.x - self.min.x
    }

    fn y_range(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn convert_to_pixel<G: Into<GraphCoord>>(&self, gp: G) -> Option<Pixel> {
        let gp = gp.into();

        if self.off_grid(gp) {
            return None;
        }

        let x_origin_pixel = self.horizontal_border();
        let new_x = x_origin_pixel + (self.actual_width() * ((gp.x - self.min.x) / self.x_range()));
        
        let y_origin_pixel = self.vertical_border();
        let new_y = y_origin_pixel + (self.actual_height() * ((gp.y - self.min.y) / self.y_range()));

        Some(Pixel::new(new_x, new_y))
    }

    pub fn convert_to_graphcoord<P: Into<Pixel>>(&self, p: P) -> Option<GraphCoord> {
        let p = p.into();

        if p.x < self.horizontal_border || p.x > (self.width - self.horizontal_border) {
            return None;
        } 
        if p.y < self.vertical_border() || p.y > (self.height - self.vertical_border()) {
            return None;
        }

        let GraphCoord { x, y } = self.distance_travelled_to_relative(p);
        Some(Pixel::new(x + self.min.x, y + self.min.y))
    }

    pub fn distance_travelled_to_relative<P: Into<Pixel>>(&self, p: P) -> GraphCoord {
        let p = p.into();

        let percent_x = ((p.x - self.horizontal_border()) / self.actual_width()) * self.x_range();
        let percent_y = ((p.y - self.vertical_border()) / self.actual_height()) * self.y_range();

        GraphCoord::new(percent_x, percent_y)
    }


    pub fn adjust_for(&mut self, ds: &DataSet) {
        let max = utils::get_max_coord(&[self.max, ds.get_max_coord()]);
        let min = utils::get_min_coord(&[self.min, ds.get_min_coord()]);

        self.max = max;
        self.min = min;
    }

    pub fn adjust_for_axis(&mut self, axis: &Axis) {
        let x_max = axis.x_label.max;
        let x_min = axis.x_label.min;

        let y_max = axis.y_label.max;
        let y_min = axis.y_label.min;

        self.max = GraphCoord::new(x_max, y_max);
        self.min = GraphCoord::new(x_min, y_min);
    }

    pub fn off_grid<G: Into<GraphCoord>>(&self, g: G) -> bool {
        let GraphCoord { x, y } = g.into();

        x > self.max.x || x < self.min.x || y > self.max.y || y < self.min.y
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

        assert_eq!(dim.convert_to_pixel((0.0, 0.0)), Some(Pixel::new(300.0, 300.0)));
        assert_eq!(dim.convert_to_pixel((-3.0, 5.0)), Some(Pixel::new(228.0, 420.0)));

        // with just positive x and y
        dim.min = GraphCoord::new(0.0, 0.0);
        assert_eq!(dim.convert_to_pixel((2.0, 1.0)), Some(Pixel::new(156.0, 108.0)));

        
        // Test horizontal_border and vertical_border settings
        dim.horizontal_border = 0.2;
        dim.vertical_border = 0.3;

        assert_eq!(dim.convert_to_pixel((2.0, 1.0)), Some(Pixel::new(192.0, 204.0)));

        // convert_to_pixel should return None on out of bounds
        assert_eq!(dim.convert_to_pixel((20.0, -3.0)), None);

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