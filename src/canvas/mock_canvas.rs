use pixel::{Pixel, Color};
use canvas::Canvas;

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct MockError(pub String);

pub struct MockCanvas {
    pub shown: i32,
    pub cleared: i32,
    pub color: Color
}

impl MockCanvas {
    pub fn new() -> MockCanvas {
        MockCanvas {
            shown: 0,
            cleared: 0,
            // unlikely to be this by random so we can check
            // if it's changed
            color: Color(1, 1, 1) 
        }
    }
}

#[allow(unused_variables, dead_code)]
impl Canvas for MockCanvas {
    type Err = MockError;

    fn get_origin(&self) -> Pixel {
        Pixel::new(0.0, 0.0)
    }
    /// Should return the actual size of the canvas created in pixels
    fn get_size(&self) -> (f64, f64) {
        (600.0, 600.0)
    }

    /// Should draw a line from `start` to `end`
    fn draw_line<P: Into<Pixel>>(&mut self, start: P, end: P) -> Result<(), Self::Err> {
        Ok(())
    }
    /// Should outline the rect in active color
    fn draw_rect<P: Into<Pixel>>(&mut self, start: P, width: f64, height: f64) -> Result<(), Self::Err> {
        Ok(())
    }
    /// Should fill the rect in active color
    fn fill_rect<P: Into<Pixel>>(&mut self, start: P, width: f64, height: f64) -> Result<(), Self::Err> {
        Ok(())
    }

    /// Should write text starting at bottom_left
    fn write_text<P: Into<Pixel>>(&mut self, t: &str, bottom_left: P) -> Result<(), Self::Err> {
        if t == "fail" { 
            Err(MockError("write_text failed".to_string()))
        } else {
            Ok(())
        }
    }
    fn write_text_centred<P: Into<Pixel>>(&mut self, t: &str, centre: P) -> Result<(), Self::Err> {
        Ok(())
    }
    /// Convenience method to save converting strings to num for axis labels
    fn write_num_centred<P: Into<Pixel>>(&mut self, t: f64, p: P) -> Result<(), Self::Err> {
        Ok(())
    }

    /// Should fill the screen with the currently active color
    fn clear(&mut self) {
        self.cleared += 1;
    }
    /// Should show any changes that have been made to the canvas
    fn show(&mut self) {
        self.shown += 1;
    }

    /// Should set the currently active color of the canvas
    /// Anything drawn by the canvas should be done in the active color
    fn set_color<C: Into<Color>>(&mut self, color: C) {
        self.color = color.into();
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MockError {
    fn description(&self) -> &str {
        &*self.0
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}