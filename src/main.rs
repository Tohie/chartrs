extern crate chartrs;

use chartrs::canvas::with_sdl2_context;
use chartrs::options::{DataSetOptions, PlotStyle, AxisOptions};
use chartrs::pixel;
use chartrs::DataSet;

fn main() {
    let font_size = 12;
    let (w, h) = (800, 600);

    let x_options = AxisOptions::new().label("t (s)");
    let y_options = AxisOptions::new().label("A (V)");

    let line_options = DataSetOptions::new()
        .plot_style(PlotStyle::Line)
        .color(pixel::Color(255, 0, 0))
        .name("Sin");

    let line_opt2 = DataSetOptions::new()
        .plot_style(PlotStyle::Line)
        .color(pixel::Color(0, 0, 255))
        .name("Cos");

    let x1 = (-500..501).map(|x| (x as f64) / 100.0).collect::<Vec<f64>>();
    let x2 = x1.clone();

    let ds1 = DataSet::from_fn(x1, &line_options, |x| x.sin());
    let ds2 = DataSet::from_fn(x2, &line_opt2, |x| (x.cos() * 2.0).powi(3));

    let data_sets = vec!(&ds1, &ds2);

    with_sdl2_context(w, h, font_size, data_sets, &x_options, &y_options);
}