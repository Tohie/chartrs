extern crate sdl2;
extern crate chartrs;

use chartrs::canvas::with_sdl2_context;
use chartrs::pixel::Color;
use chartrs::options::{DataSetOptions, PlotStyle, PointStyle, AxisOptions};
use chartrs::{Graph2D, DataSet};
use std::{thread, time};

fn main() {
    let font_size = 12;
    with_sdl2_context(800, 600, font_size, |ctx| {
        let x_options = AxisOptions::new().label("t (s)");
        let y_options = AxisOptions::new().label("A (V)");

        let bar_options = DataSetOptions::new()
            .plot_style(PlotStyle::Bar)
            .random_color(true);

        let x1 = (-25..25).map(|x| x as f64).collect::<Vec<f64>>();

        let ds1 = DataSet::from_fn(x1, &bar_options, |x| x.powi(2));

        let data_sets = vec!(&ds1);
        let mut g1 = Graph2D::new(ctx, data_sets);
        g1.show(&x_options, &y_options);
    });
}