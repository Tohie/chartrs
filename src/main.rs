extern crate sdl2;
extern crate chartrs;

use chartrs::canvas::with_sdl2_context;
use chartrs::pixel::Color;
use chartrs::options::{DataSetOptions, PlotStyle, PointStyle, AxisOptions};
use chartrs::{Graph2D, DataSet};

fn main() {
    let font_size = 12;
    with_sdl2_context(800, 600, font_size, |ctx| {
        let x_options = AxisOptions::new().label("t (s)");
        let y_options = AxisOptions::new().label("A (V)");

        let line_options = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(255, 0, 0));

        let scatter_options = DataSetOptions::new()
            .plot_style(PlotStyle::Scatter)
            .point_style(PointStyle::Cross)
            .random_color(true);

        let bar_options = DataSetOptions::new()
            .plot_style(PlotStyle::Bar)
            .color(Color(0, 0, 255));

        let x1 = (-400..401).map(|x| (x as f64)/10.0).collect::<Vec<f64>>();
        let x2 = (-9..10).map(|x| x as f64).collect::<Vec<f64>>(); 
        let x3 = x2.clone();

        let ds1 = DataSet::from_fn(x1, &line_options, |x| x * -2.0);
        let ds2 = DataSet::from_fn(x2, &scatter_options, |x| x.powi(2));
        let ds3 = DataSet::from_fn(x3, &bar_options, |x| x + 2.0 );

        let data_sets = &[&ds1, &ds2, &ds3];
        let mut g1 = Graph2D::new(ctx, data_sets, &x_options, &y_options);

        g1.show();
    });
}