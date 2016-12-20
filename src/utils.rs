use pixel::GraphCoord;

pub fn get_max_coord(coords: &[GraphCoord]) -> GraphCoord {
        find_x_and_y_by_predicate(coords, |x_curr, x_max| x_curr > x_max)
    }

pub fn get_min_coord(coords: &[GraphCoord]) -> GraphCoord {
    find_x_and_y_by_predicate(coords, |x_curr, x_min| x_curr < x_min)
}

fn find_x_and_y_by_predicate<F>(coords: &[GraphCoord], f: F) -> GraphCoord
    where F: Fn(f64, f64) -> bool {

    coords.iter().fold(GraphCoord::new(0.0, 0.0), |acc, px| {
        let x = if f(px.x, acc.x) { px.x } else { acc.x };
        let y = if f(px.y, acc.y) { px.y } else { acc.y } ;
        GraphCoord::new(x, y)
    })
}
