use sdl2;
use sdl2::render::{Renderer, TextureQuery};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::ttf::Font;
use sdl2::keyboard::Keycode;

use canvas::Canvas;
use pixel;
use pixel::Pixel;
use graph_2d::Graph2D;
use options::AxisOptions;
use data_set::DataSet;
use canvas::sdl2::SDL2Error;

use std::path::Path;

/// `SDL2Canvas` is a struct that holds an sdl2 font and renderer
/// which are required to fully implement the `Canvas` trait
/// See the `Canvas` trait documentation for an explanation of what these functions
/// do.
pub struct SDL2Canvas<'a> {
    renderer: Renderer<'a>,
    font: Font<'a>,
}

impl <'a> SDL2Canvas<'a> {
    pub fn new(renderer: Renderer<'a>, font: Font<'a>) -> SDL2Canvas<'a> {
        SDL2Canvas { renderer: renderer, font: font }
    }

    // SDL2 uses top left as origin, Axis2D assumes a bottom left
    // origin
    fn convert_to_bottom_left_origin<P: Into<Pixel>>(&self, p: P) -> Pixel {
        let (_, h) = self.get_size();
        let p = p.into();

        Pixel::new(p.x, (h - p.y).abs())
    }
}

impl <'a> Canvas for SDL2Canvas<'a> {
    type Err = SDL2Error;

    fn get_origin(&self) -> Pixel {
        Pixel::new(0.0, 0.0)
    }

    fn get_size(&self) -> (f64, f64) {
        let point = self.renderer.viewport().bottom_right();
        (point.x() as f64, point.y() as f64)
    }

    fn draw_line<P: Into<Pixel>>(&mut self, start: P, end: P) -> Result<(), SDL2Error> {
        let Pixel { x: x1, y: y1 } = self.convert_to_bottom_left_origin(start);
        let Pixel { x: x2, y: y2 } = self.convert_to_bottom_left_origin(end);

        self.renderer.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32))?;

        Ok(())
    }

    fn draw_rect<P: Into<Pixel>>(&mut self, start: P, width: f64, height: f64) -> Result<(), SDL2Error> {
        let Pixel { x, y } = self.convert_to_bottom_left_origin(start);
        let rect = Rect::new(x as i32, (y - height) as i32, width as u32, height as u32);
        self.renderer.draw_rect(rect)?;

        Ok(())
    }

    fn fill_rect<P: Into<Pixel>>(&mut self, start: P, width: f64, height: f64) -> Result<(), SDL2Error> {
        let Pixel { x, y } = self.convert_to_bottom_left_origin(start);
        let rect = Rect::new(x as i32, (y - height) as i32, width as u32, height as u32);
        self.renderer.fill_rect(rect)?;

        Ok(())
    }

    fn write_text<P: Into<Pixel>>(&mut self, t: &str, bottom_left: P) -> Result<(), SDL2Error> {
        let surface = self.font.render(t).blended(Color::RGB(0, 0, 0))?;
        let texture = self.renderer.create_texture_from_surface(&surface)?;

        let TextureQuery { width, height, .. } = texture.query();
        let pix = self.convert_to_bottom_left_origin(bottom_left);
        let r = Rect::new(pix.x as i32, (pix.y as i32 - height as i32), width, height);
        self.renderer.copy(&texture, None, Some(r))?;

        Ok(())
    }
    
    fn write_text_centred<P: Into<Pixel>>(&mut self, t: &str, centre: P) -> Result<(), SDL2Error> {
        let surface = self.font.render(t).blended(Color::RGB(0, 0, 0))?;
        let texture = self.renderer.create_texture_from_surface(&surface)?;

        let TextureQuery { width, height, .. } = texture.query();
        let pix = self.convert_to_bottom_left_origin(centre);
        let centre_x = pix.x - (width as f64 / 2.0);
        let centre_y = pix.y - (height as f64 / 2.0);
        let r = Rect::new(centre_x as i32, centre_y as i32, width, height);
        self.renderer.copy(&texture, None, Some(r))?;

        Ok(())
    }
    
    fn clear(&mut self) {
        self.renderer.clear();
    }

    fn show(&mut self) {
        self.renderer.present();
    }

    fn set_color<C: Into<pixel::Color>>(&mut self, color: C) {
        let pixel::Color(r, g, b) = color.into();
        self.renderer.set_draw_color(Color::RGB(r, g, b));
    }
}

/// This is a convenience function
/// It will construct a window with given width and height
/// then pass an `SDL2Canvas` to the function given which allows a graph to be
/// drawn, it will then loop until the window is closed or esc is pressed
pub fn plot<'a, 'c, 'o, A>(w: u32, h: u32, font_size: u16,
    data_sets: Vec<&'a DataSet<'a>>, x_opts: A, y_opts: A) -> Result<(), SDL2Error>
    where A: Into<Option<&'o AxisOptions<'o>>> {

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

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

    let font = ttf_context.load_font(Path::new("./Ubuntu-R.ttf"), font_size).unwrap();
    let mut canvas = SDL2Canvas::new(renderer, font);

    let mut graph = Graph2D::with_axises(&mut canvas, data_sets, x_opts, y_opts);
    graph.show()?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut prev_x = -1.0;
    let mut prev_y = -1.0;
    let mut mouse_down = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            mouse_down = true;
                            prev_x = x as f64;
                            prev_y = y as f64;
                        },
                        _ => {},
                    };
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => mouse_down = false,
                        _ => {},
                    };
                },
                Event::MouseMotion { x, y, .. } => {
                    if mouse_down {
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
                            (Some(_), Some(_)) => {
                                let delta = graph.dimensions.distance_travelled_to_relative((prev_x - x, y - prev_y));
                                graph.move_view(delta.x, delta.y)?;
                            },
                            _ => {},
                        }

                        prev_x = x;
                        prev_y = y;
                    }
                },
                _ => {}
            }
        }
    }

    Ok(())
}