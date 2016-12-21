//! This module defines the `Canvas` trait
//! Anything that implements this crate correctly
//! can be used by Graph2D to draw a graph on
//! see the SDL2 implementation for an example

pub mod sdl2_canvas;

pub use self::sdl2_canvas::with_sdl2_context;

use pixel::{Pixel, Color};

/// Anything that implements `Canvas` can be used to draw a graph on
/// A `Canvas` origin should *always* be the bottom left corner
/// Positive x should be to the right and positive y should be up
pub trait Canvas {
    fn get_origin(&self) -> Pixel;
    /// Should return the actual size of the canvas created in pixels
    fn get_size(&self) -> (f64, f64);

    /// Should draw a line from `start` to `end`
    fn draw_line<P: Into<Pixel>>(&mut self, start: P, end: P);
    
    /// Should write text `t` with the width and height defined by the font type
    /// and the `centre` should be where the centre of the text drawn 
    /// is in pixels 
    fn write_text<P: Into<Pixel>>(&mut self, t: &str, centre: P);
    /// Convenience method to save converting strings to num for axis labels
    fn write_num<P: Into<Pixel>>(&mut self, t: f64, p: P) {
        let t = format!("{}", t);
        self.write_text(&t, p);
    }

    /// Should fill the screen with the currently active color
    fn clear(&mut self);
    /// Should show any changes that have been made to the canvas
    fn show(&mut self);

    /// Should set the currently active color of the canvas
    /// Anything drawn by the canvas should be done in the active color
    fn set_color<C: Into<Color>>(&mut self, color: C);
}