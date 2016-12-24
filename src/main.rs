extern crate sdl2;
extern crate chartrs;

use sdl2::render::{Renderer, TextureQuery};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseState;
use sdl2::ttf::Font;
use sdl2::keyboard::Keycode;

use chartrs::canvas::with_sdl2_context;
use chartrs::canvas::sdl2_canvas::SDL2Canvas;
use chartrs::pixel::{GraphCoord, Pixel};
use chartrs::pixel;
use chartrs::options::{DataSetOptions, PlotStyle, AxisOptions};
use chartrs::{Graph2D, DataSet};

use std::thread;
use std::time::Duration;
use std::path::Path;

fn main() {
    let font_size = 12;
    /*
    with_sdl2_context(800, 600, font_size, |ctx| {
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
        let mut g1 = Graph2D::with_axises(ctx, data_sets, &x_options, &y_options);
        g1.show();
    });
    */
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (w, h) = (800, 600);
    let window = video_subsystem.window("rust-sdl2 demo: Video", w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();
    renderer.set_draw_color(Color::RGB(0, 0, 0));

    let font_size = 12;
    let font = ttf_context.load_font(Path::new("./Ubuntu-R.ttf"), font_size).unwrap();
    let mut canvas = SDL2Canvas::new(renderer, font);

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
    let mut graph = Graph2D::with_axises(&mut canvas, data_sets, &x_options, &y_options);
    graph.show();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut prev_x = -1.0;
    let mut prev_y = -1.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion { mousestate: ms, x, y, .. } => {
                    if ms.left() {
                        let x = x as f64;
                        let y = y as f64;
                        if prev_x == -1.0 || prev_y == -1.0 {
                            prev_x = x;
                            prev_y = y;
                            continue 'running;
                        }
                        let curr_coord = graph.dimensions.convert_to_graphcoord((x, y));
                        let prev_coord = graph.dimensions.convert_to_graphcoord((prev_x, prev_y));

                        match (curr_coord, prev_coord) {
                            (Some(curr), Some(prev)) => {
                                let delta = graph.dimensions.distance_travelled_to_relative((x - prev_x, y - prev_y));
                                graph.move_view(delta.x, delta.y);
                            },
                            _ => {},
                        }

                        prev_x = x;
                        prev_y = y;
                    }
                }
                _ => {}
            }
        }
    }
}