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


// returns the upper and lower limits and the increment size
pub fn pretty_axis_values(max: f64, min: f64, tick_count: f64) -> (f64, f64, f64) {
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