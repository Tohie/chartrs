extern crate sdl2;
extern crate chartrs;

use chartrs::canvas::with_sdl2_context;
use chartrs::pixel::Color;
use chartrs::options::{DataSetOptions, PlotStyle, AxisOptions};
use chartrs::{Graph2D, DataSet};
use std::thread;
use std::time::Duration;

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
        let ds2 = DataSet::from_fn(x2, &line_opt2, |x| (x.cos() * 2.0).powi(3));

        let data_sets = vec!(&ds1, &ds2);
        let mut g1 = Graph2D::with_axises(ctx, data_sets, &x_options, &y_options);
        g1.show();

        let mut t = 0.0;
        let x = 1.0/60.0;
        let y = -2.0/60.0;
        let fps = Duration::from_millis(17);
        'main: loop {
            t += 1.0/60.0;
            g1.move_view(x, y);
            if (t as i32) > 2 {
                break 'main;
            }
            thread::sleep(fps);
        }
        g1.show();
    });
}