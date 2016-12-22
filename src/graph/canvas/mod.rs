mod graph_canvas;

pub use self::graph_canvas::GraphCanvas;

use pixel::{GraphCoord, Pixel};

#[derive(Clone, Copy, PartialEq)]
pub struct GraphBounds {
    max_x: f64,
    max_y: f64,

    min_x: f64,
    min_y: f64,

    height: f64,
    width: f64,

    horizontal_border: f64,
    vertical_border: f64,
}

impl GraphBounds {
    pub fn new<G: Into<GraphCoord>>(min: G, max: G, width: f64, height: f64) -> GraphBounds {
        let min = min.into();
        let max = max.into();
        GraphBounds {
            max_x: max.x,
            max_y: max.y,

            min_x: min.x,
            min_y: min.y,

            height: height,
            width: width,

            horizontal_border: 0.1,
            vertical_border: 0.1,
        }
    }

    pub fn convert_to_pixel<G: Into<GraphCoord>>(&self, gp: G) -> Pixel {
        let gp = gp.into();
        let origin = Pixel::new(0.0, 0.0);
        let horizontal_border = self.horizontal_border;
        let vertical_border = self.vertical_border;

        let x_range = self.max_x - self.min_x;
        let y_range = self.max_y - self.min_y;

        let x_origin_pixel = origin.x + (self.width * horizontal_border);
        let actual_width_pixels = self.width - (2.0 * self.width * horizontal_border);
        let new_x = x_origin_pixel + (actual_width_pixels * ((self.min_x.abs() + gp.x) / x_range));
        
        let y_origin_pixel = origin.y + (self.height * vertical_border);
        let actual_height_pixels = self.height - (2.0 * self.height * vertical_border);
        let new_y = y_origin_pixel + (actual_height_pixels * ((self.min_y.abs() + gp.y) / y_range));

        Pixel::new(new_x, new_y)
    }
}