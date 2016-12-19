extern crate sdl2;

pub mod graph;
pub mod pixel;
pub mod canvas;

pub use pixel::Pixel;
pub use canvas::Canvas;
pub use graph::{Graph2D, DataSet, Axis2D, Graph, PlotStyle, PointStyle};


#[cfg(test)]
mod tests {
    use canvas::Canvas;
    use pixel::Pixel;

    pub struct FakeCanvas {}

    impl Canvas for FakeCanvas {
        fn get_origin(&self) -> Pixel {
            Pixel { x: 0f64, y: 0f64, }
        }
        
        fn get_size(&self) -> (f64, f64) {
            (600, 800)
        }

        fn clear (&mut self) {}
        fn draw_line(&self, start: Pixel, end: Pixel) {}
        fn show(&mut self) {}
        fn set_color(&mut self, r: u8, g: u8, b: u8) {}
    }
}
