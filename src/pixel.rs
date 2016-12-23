//! This module defines types that will be used to create common types
//! such as pixels or colours, in order to make arguments to functions
//! clearer

use rand::{Rand, Rng};

/// The Pixel type, this represents an (x, y) location on the screen
/// relative to the bottom left of the screen
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub x: f64,
    pub y: f64,
}

/// A GraphCoord represents an (x, y) point on a graph and needs to be 
/// converted a Pixel by an Axis
pub type GraphCoord = Pixel;

impl Pixel {
    pub fn new(x: f64, y: f64) -> Pixel {
        Pixel {x: x, y: y}
    }
}

impl Into<Pixel> for (f64, f64) {
    fn into(self) -> Pixel {
        Pixel::new(self.0, self.1)
    } 
}

#[derive(Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Into<Color> for (u8, u8, u8) {
    fn into(self) -> Color {
        Color(self.0, self.1, self.2)
    }
}

impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let r = rng.gen::<u8>();
        let g = rng.gen::<u8>();
        let b = rng.gen::<u8>();

        Color(r, g, b)
    }
}