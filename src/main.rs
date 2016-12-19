extern crate sdl2;
extern crate graphrs;

use graphrs::canvas::with_sdl2_renderer;
use graphrs::{Graph2D, PlotStyle};

fn main() {
    with_sdl2_renderer(800, 600, |c| {
        let xs = (-400..401).map(|x| (x as f64) / 100.0).collect::<Vec<f64>>();

        Graph2D::plot_fn(c, PlotStyle::Line, xs, |x| x.powi(2));
    });
}