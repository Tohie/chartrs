use pixel::{GraphCoord, Color};
use graph::DataSetOptions;
use utils;
use rand;
use rand::Rng;

/// `DataSet` represents a series on a 2D graph
/// `DataSet` holds a vector of x and y co-ordinates for a graph
/// as well as a series of options such as the colour to draw the line
pub struct DataSet<'a> {
    pub data_points: Vec<GraphCoord>,
    pub options: &'a DataSetOptions<'a>,
}

impl <'a> DataSet<'a> {
    /// Takes vector of x co-ordinates as well as options and then uses the given 
    /// function f to create a vector of `GraphCoord`
    pub fn from_fn<F>(x: Vec<f64>, opts: &'a DataSetOptions<'a>, f: F) -> Self 
        where F: Fn(f64) -> f64 {
        
        let xc = x.clone();
        let y = xc.into_iter().map(f);
        let pixels = x.into_iter().zip(y).map(|(x, y)| GraphCoord { x: x, y: y}).collect::<Vec<_>>();

        DataSet {
            data_points: pixels,
            options: opts,
        }
    }

    /// Gets the maximum value of x and maximum value of y
    /// and returns them as a `GraphCoord`
    pub fn get_max_coord(&self) -> GraphCoord {
        utils::get_max_coord(&self.data_points)
    }

    pub fn get_min_coord(&self) -> GraphCoord {
        utils::get_min_coord(&self.data_points)
    }

    /// Using the options specified in the options field
    /// chooses the color that should be used to draw this
    /// `DataSet`
    pub fn choose_color(&self) -> Color {
        if !self.options.random_color {
            self.options.color
        } else {
            let mut rng = rand::thread_rng();
            match self.options.colors {
                Some(choices) => *rng.choose(choices).unwrap_or(&Color(0, 0, 0)),
                None => rng.gen::<Color>(),
            }
        }    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fn_test() {
        let ds = DataSet::from_fn(vec!(1.0, 2.0, 3.0, -1.0), |x| x*2.0);

        assert_eq!(ds.data_points[0].x, 1.0);
        assert_eq!(ds.data_points[1].x, 2.0);
        assert_eq!(ds.data_points[2].x, 3.0);
        assert_eq!(ds.data_points[3].x, -1.0);

        for data_point in ds.data_points.iter() {
            assert_eq!(data_point.x*2.0, data_point.y);
        }

    }

    #[test]
    fn max_x_and_y_test() {
        let ds = DataSet::from_fn(vec!(1.0, 2.0, 3.0, -1.0), |x| x*2.0);

        let (max_x, max_y) = ds.get_max_x_and_y();
        assert_eq!(max_x, 3.0);
        assert_eq!(max_y, 6.0);
    }

    #[test]
    fn min_x_and_y_test() {
        let ds = DataSet::from_fn(vec!(1.0, 2.0, 3.0, -1.0), |x| x*2.0);

        let (max_x, max_y) = ds.get_min_x_and_y();
        assert_eq!(max_x, -1.0);
        assert_eq!(max_y, -2.0);
    }
}