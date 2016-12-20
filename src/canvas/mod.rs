pub mod sdl2_canvas;

pub use self::sdl2_canvas::with_sdl2_context;

use pixel::{Pixel, Color};

pub trait Canvas {
    fn get_origin(&self) -> Pixel;
    fn get_size(&self) -> (f64, f64);

    fn draw_line<P: Into<Pixel>>(&mut self, start: P, end: P);
    fn write_text<P: Into<Pixel>>(&mut self, t: &str, bottom_left_corner: P);
    fn write_num<P: Into<Pixel>>(&mut self, t: f64, p: P) {
        let t = format!("{}", t);
        self.write_text(&t, p);
    }

    fn clear(&mut self);
    fn show(&mut self);

    fn set_color<C: Into<Color>>(&mut self, color: C);
}