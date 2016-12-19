use pixel::GraphCoord;
use graph::PointStyle;

pub struct DataSet  {
    pub data_points: Vec<GraphCoord>,
    pub point_style: PointStyle,
}

impl DataSet {
    pub fn from_fn<F>(x: Vec<f64>, f: F) -> Self 
        where F: Fn(f64) -> f64 {
        
        let xc = x.clone();
        let y = xc.into_iter().map(f);
        let pixels = x.into_iter().zip(y).map(|(x, y)| GraphCoord { x: x, y: y}).collect::<Vec<_>>();

        DataSet {
            data_points: pixels,
            point_style: PointStyle::Nothing,
        }
    }

    pub fn get_max_x_and_y(&self) -> (f64, f64) {
        self.find_x_and_y_by_predicate(|x_curr, x_max| x_curr > x_max)
    }

    pub fn get_min_x_and_y(&self) -> (f64, f64) {
        self.find_x_and_y_by_predicate(|x_curr, x_min| x_curr < x_min)
    }

    fn find_x_and_y_by_predicate<F>(&self, f: F) -> (f64, f64)
        where F: Fn(f64, f64) -> bool {

        self.data_points.iter().fold((0.0, 0.0), |(x, y), px| {
            (if f(px.x, x) { px.x } else { x }, if f(px.y, y) { px.y } else { y })
        })
    }

    pub fn set_point_style<'a>(&'a mut self, point_style: PointStyle) -> &'a mut Self {
        self.point_style = point_style;
        self
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