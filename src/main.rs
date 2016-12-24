extern crate sdl2;
extern crate chartrs;

use chartrs::canvas::with_sdl2_context;
use chartrs::pixel::Color;
use chartrs::options::{DataSetOptions, PlotStyle, AxisOptions};
use chartrs::{Graph2D, DataSet};

fn main() {
    let font_size = 12;
    with_sdl2_context(800, 600, font_size, |ctx| {
        let x_options = AxisOptions::new().label("t (s)");
        let y_options = AxisOptions::new().label("A (V)");

        let line_options = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(255, 0, 0));

        let x1 = (-25..25).map(|x| (x as f64).powi(2) as f64).collect::<Vec<f64>>();

        let ds1 = DataSet::from_fn(x1, &line_options, |x| x.sin());

        let data_sets = vec!(&ds1);
        let mut g1 = Graph2D::new(ctx, data_sets);
        g1.show(&x_options, &y_options);
    });
}