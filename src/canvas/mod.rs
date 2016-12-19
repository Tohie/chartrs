pub mod sdl2_canvas;

pub use self::sdl2_canvas::with_sdl2_renderer;

use pixel::Pixel;

pub trait Canvas {
    fn get_origin(&self) -> Pixel;
    fn get_size(&self) -> (f64, f64);

    fn draw_line<P: Into<Pixel>>(&mut self, start: P, end: P);
    fn write_text<P: Into<Pixel>>(&mut self, t: &str, bottom_left_corner: P);

    fn clear(&mut self);
    fn show(&mut self);

    fn set_color(&mut self, r: u8, g: u8, b: u8);
}