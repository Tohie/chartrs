/// The Pixel type, this represents an (x, y) location on the screen
/// relative to the bottom left of the screen
#[derive(Debug, Copy, Clone)]
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
