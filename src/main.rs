extern crate sdl2;
extern crate chartrs;

use chartrs::canvas::with_sdl2_context;
use chartrs::pixel::Color;
use chartrs::{Graph2D, PlotOptions, DataSetOptions, PlotStyle, PointStyle, DataSet, Graph};

fn main() {
    let font_size = 12;
    with_sdl2_context(800, 600, font_size, |ctx| {
        let plot_options = PlotOptions::new()
            .tick_count(8.0)
            .x_label("t (s)")
            .y_label("A (V)");

        let line_options = DataSetOptions::new()
            .plot_style(PlotStyle::Line)
            .color(Color(255, 0, 0));

        let scatter_options = DataSetOptions::new()
            .plot_style(PlotStyle::Scatter)
            .point_style(PointStyle::Cross)
            .random_color(true);

        let x1 = (-400..401).map(|x| (x as f64)/10.0).collect::<Vec<f64>>();
        let x2 = (-9..10).map(|x| x as f64).collect::<Vec<f64>>(); 

        let ds1 = DataSet::from_fn(x1, &line_options, |x| x * -2.0);
        let ds2 = DataSet::from_fn(x2, &scatter_options, |x| x.powi(2));
        
        let data_sets = &[&ds1, &ds2];
        let mut g1 = Graph2D::new(ctx, data_sets, plot_options);

        g1.plot();
    });
}