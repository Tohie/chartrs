//! This module defines the `Canvas` trait
//! Anything that implements this crate correctly
//! can be used by Graph2D to draw a graph on
//! see the SDL2 implementation for an example

pub mod sdl2;

use pixel::{Pixel, Color};

use std::error::Error;

/// Anything that implements `Canvas` can be used to draw a graph on
/// A `Canvas` origin should *always* be the bottom left corner
/// Positive x should be to the right and positive y should be up
pub trait Canvas {
    type Err: Error;

    fn get_origin(&self) -> Pixel;
    /// Should return the actual size of the canvas created in pixels
    fn get_size(&self) -> (f64, f64);

    /// Should draw a line from `start` to `end`
    fn draw_line<P: Into<Pixel>>(&mut self, start: P, end: P) -> Result<(), Self::Err>;
    /// Should outline the rect in active color
    fn draw_rect<P: Into<Pixel>>(&mut self, start: P, width: f64, height: f64) -> Result<(), Self::Err>;
    /// Should fill the rect in active color
    fn fill_rect<P: Into<Pixel>>(&mut self, start: P, width: f64, height: f64) -> Result<(), Self::Err>;

    /// Should write text starting at bottom_left
    fn write_text<P: Into<Pixel>>(&mut self, t: &str, bottom_left: P) -> Result<(), Self::Err>;
    fn write_text_centred<P: Into<Pixel>>(&mut self, t: &str, centre: P) -> Result<(), Self::Err>;
    /// Convenience method to save converting strings to num for axis labels
    fn write_num_centred<P: Into<Pixel>>(&mut self, t: f64, p: P) -> Result<(), Self::Err> {
        let t = format!("{:.1}", t);
        self.write_text_centred(&t, p)
    }

    /// Should fill the screen with the currently active color
    fn clear(&mut self);
    /// Should show any changes that have been made to the canvas
    fn show(&mut self);

    /// Should set the currently active color of the canvas
    /// Anything drawn by the canvas should be done in the active color
    fn set_color<C: Into<Color>>(&mut self, color: C);
}