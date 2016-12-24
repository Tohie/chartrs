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
            .color(Color(255, 0, 0))
            .name("Sin");

        let line_opt2 = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(0, 0, 255))
            .name("Cos");

        let x1 = (0..1001).map(|x| (x as f64) / 100.0).collect::<Vec<f64>>();
        let x2 = x1.clone();

        let ds1 = DataSet::from_fn(x1, &line_options, |x| x.sin());
        let ds2 = DataSet::from_fn(x2, &line_opt2, |x| x.cos());

        let data_sets = vec!(&ds1, &ds2);
        let mut g1 = Graph2D::new(ctx, data_sets);
        
        g1.show(&x_options, &y_options);
    });
}