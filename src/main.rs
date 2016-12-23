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

        let line_red = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(255, 0, 0));

        let line_blue = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(0, 0, 255));

        let line_green = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(0, 255, 0));

        let x1 = (-20000..20000).map(|x| (x as f64)/1000.0).collect::<Vec<f64>>();
        let x2 = x1.clone();
        let x3 = x1.clone();

        let ds1 = DataSet::from_fn(x1, &line_red, |x| -(x.powi(2)));
        let ds2 = DataSet::from_fn(x2, &line_blue, |x| x.powi(2));
        let ds3 = DataSet::from_fn(x3, &line_green, |x| 400.0 * x.sin());

        let data_sets = vec!(&ds1);
        let mut g1 = Graph2D::new(ctx, data_sets);
        g1.show(&x_options, &y_options);

        let two_seconds = time::Duration::from_secs(2);
        thread::sleep(two_seconds);

        g1.add_data_set(&ds2);
        g1.show(&x_options, &y_options);

        thread::sleep(two_seconds);

        g1.add_data_set(&ds3);
        g1.show(&x_options, &y_options);
    });
}