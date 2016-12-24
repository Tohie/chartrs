use pixel::{GraphCoord, Color};
use options::DataSetOptions;
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
    pub fn new(data_points: Vec<GraphCoord>, opts: &'a DataSetOptions<'a>) -> Self {
        DataSet {
            data_points: data_points,
            options: opts,
        }
    }

    pub fn from_vecs(x: Vec<f64>, y: Vec<f64>, opts: &'a DataSetOptions<'a>) -> Option<Self> {
        if x.len() != y.len() {
            return None;
        }

        let pixels = x.into_iter().zip(y).map(|(x, y)| GraphCoord::new(x, y)).collect::<Vec<_>>();
        Some(DataSet::new(pixels, opts))
    }

    /// Takes vector of x co-ordinates as well as options and then uses the given 
    /// function f to create a vector of `GraphCoord`
    pub fn from_fn<F>(x: Vec<f64>, opts: &'a DataSetOptions<'a>, f: F) -> Self 
        where F: Fn(f64) -> f64 {
        
        let xc = x.clone();
        let y = xc.into_iter().map(f).collect::<Vec<f64>>();
        DataSet::from_vecs(x, y, opts).expect("y.len() must equal x.len()")
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