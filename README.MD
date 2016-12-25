# ChartRS

This library can be used for plotting basic 2D charts such as a bar, line or scatter graph

Please note, this library is nowhere near stable at the moment and likely has a few bugs
This library also requires a font file, please change the font-path in src/canvas/sdl2_canvas.rs
to point to a ttf file, n.b. you can grab some open source font like the ubuntu font if needed.

## Current Features

* Draw Scatter, Line and Bar Graphs using SDL2
* Ability to change color of Bars, Lines or Point
* X and Y axis labels
* Multiple named series with a legend
* Calculating "pretty" axis ticks, with option to change amount of ticks
* Interactive panning of drawn graph

## Planned Features

* More 2D charts such as Pie charts and Histograms etc. 
* Subplots
* Named labels, instead of numbers for axis ticks
* Line styles, instead of just a block line style
* More convenience functions for creating graphs, once API has stabilised a bit

## Basic Example

```rust
extern crate chartrs;

use chartrs::canvas::sdl2::plot;
use chartrs::options::{DataSetOptions, PlotStyle, AxisOptions};
use chartrs::pixel;
use chartrs::DataSet;

use std::error::Error;

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

    let res = plot(w, h, font_size, data_sets, &x_options, &y_options);

    match res {
        Err(e) => match e.cause() {
            Some(cause) => println!("Error: {}, cause: {}", e.description(), cause),
            None => println!("Error: {}, unknown cause", e.description())
        },
        Ok(_) => {},
    }
}
```

The resulting graph for the code above is shown below:

![Example GIF](https://github.com/Tohie/chartrs/blob/master/assets/pan_example.gif)

## Contributing to the Library

If you wish to add any graphs to this library, then simply create a struct with any state that your graph requires such as a `DataSet` or a `GraphCoord` and implemenet `Plottable` for it. Note: the `plot` function in `Plottable` will give you a `Canvas` and a `GraphDimensions` to draw with, `GraphDimensions` can be used to convert pixel values to a `GraphCoord` and see the `Canvas` trait for the functions available for drawing. If your graph uses a primitive construct such as an arc for a Pie chart, then feel free to make that a `Plottable` struct and build your graph out of those.

If you wish to add a backend then you must implement the `Canvas` trait, it would also be nice if you could follow the SDL implementation and define an error type similar to it for your backend as well as a corresponding `plot` function.

If you spot any code that could be improved or implement one of the planned features a pull request with the changes would be much appreciated!
