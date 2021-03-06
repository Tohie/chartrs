use pixel::GraphCoord;
// use graph_dimensions::GraphDimensions;
// use plottable::primitives::Line;
use std::f64;

macro_rules! try_opt {
    ($e:expr) => (
        match $e {
            Some(res) => res,
            None => return Ok(()),
        };
    )
}

pub fn get_max_coord(coords: &[GraphCoord]) -> GraphCoord {
        find_x_and_y_by_predicate((f64::MIN, f64::MIN), coords, |x_curr, x_max| x_curr > x_max)
    }

pub fn get_min_coord(coords: &[GraphCoord]) -> GraphCoord {
    find_x_and_y_by_predicate((f64::MAX, f64::MAX), coords, |x_curr, x_min| x_curr < x_min)
}

fn find_x_and_y_by_predicate<F, G>(init: G, coords: &[GraphCoord], f: F) -> GraphCoord
    where F: Fn(f64, f64) -> bool,
          G: Into<GraphCoord> {

    coords.iter().fold(init.into(), |acc, px| {
        let x = if f(px.x, acc.x) { px.x } else { acc.x };
        let y = if f(px.y, acc.y) { px.y } else { acc.y } ;
        GraphCoord::new(x, y)
    })
}

/*
pub fn interp_to_grid(dims: &GraphDimensions, line: Line) -> Line {
    let start = line.0;
    let end = line.1;

    let max = dims.max;
    let min = dims.min;

    let m = (start.x - end.x) / (start.y - end.y);
    let c = start.y - (m*start.x);

    let start = if dims.off_grid(start) { interp(max, min, start, m, c) } else { start };
    let end = if dims.off_grid(end) { interp(max, min, end, m, c) } else { end };

    Line(start, end)
}

fn interp(max: GraphCoord, min: GraphCoord, point: GraphCoord, m: f64, c: f64) -> GraphCoord {
    if point.x > max.x {
        GraphCoord::new(max.x, m*max.x - c)
    } else if point.x < min.x {
        GraphCoord::new(min.x, m*min.x - c)
    } else if point.y > max.y {
        GraphCoord::new((max.y - c) / m, max.y)
    } else {
        GraphCoord::new((min.y - c) / m, min.y)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use pixel::GraphCoord;
    use std::f64;

    // if get_max_coord and get_min_coord are correct,
    // then logically find_x_and_y_by_predicate must be as well
    // if both fail then problem is likely in find_x_and_y_by_predicate

    // Test negatives for get_max_coord and get_min_coord
    #[test]
    fn test_negative_coords() {
        let coords = &[GraphCoord::new(-15.0, -20.0), GraphCoord::new(-30.0, -40.0)];

        assert_eq!(get_max_coord(coords), GraphCoord::new(-15.0, -20.0));
        assert_eq!(get_min_coord(coords), GraphCoord::new(-30.0, -40.0));
    }

    #[test]
    fn test_positive_coords() {
        let coords = &[GraphCoord::new(15.0, 20.0), GraphCoord::new(30.0, 40.0), GraphCoord::new(60.0, 10.0)];

        assert_eq!(get_max_coord(coords), GraphCoord::new(60.0, 40.0));
        assert_eq!(get_min_coord(coords), GraphCoord::new(15.0, 10.0));
    }

    // for the empty case, we should return the opposite extreme,
    // i.e. get_max_coord should return (f64::MIN, f64::MIN)
    // this way it will not affect anything
    #[test]
    fn test_empty_case() {
        let coords = &[];

        assert_eq!(get_max_coord(coords), GraphCoord::new(f64::MIN, f64::MIN));
        assert_eq!(get_min_coord(coords), GraphCoord::new(f64::MAX, f64::MAX));
    }
}