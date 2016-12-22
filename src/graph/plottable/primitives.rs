use canvas::Canvas;
use pixel::GraphCoord;
use graph::PointStyle;
use graph::canvas::GraphBounds;
use graph::plottable::Plottable;

pub struct Line(pub GraphCoord, pub GraphCoord);

impl Plottable for Line {
    fn plot<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
        let start = bounds.convert_to_pixel(self.0);
        let end = bounds.convert_to_pixel(self.1);

        canvas.draw_line(start, end);
    }
}

pub struct Point(pub GraphCoord, pub PointStyle);

impl Point {
    fn plot_cross<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
        let pix = bounds.convert_to_pixel(self.0);

        canvas.draw_line((pix.x + 2.5, pix.y), (pix.x-2.5, pix.y));
        canvas.draw_line((pix.x, pix.y + 2.5), (pix.x, pix.y-2.5));
    }
}

impl Plottable for Point {
    fn plot<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
        match self.1 {
            PointStyle::Cross => self.plot_cross(bounds, canvas),
            PointStyle::Nothing => {},
        }
    }
}

pub struct Bar(pub GraphCoord, pub f64, pub f64);

impl Plottable for Bar {
    fn plot<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
    
    }
}

