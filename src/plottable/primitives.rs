use canvas::Canvas;
use pixel::GraphCoord;
use options::PointStyle;
use graph_dimensions::GraphDimensions;
use plottable::Plottable;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Line(pub GraphCoord, pub GraphCoord);

impl Plottable for Line {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let start = try_opt!(bounds.convert_to_pixel(self.0));
        let end = try_opt!(bounds.convert_to_pixel(self.1));
        
        canvas.draw_line(start, end);
    }
}

pub struct Point(pub GraphCoord, pub PointStyle);

impl Point {
    fn plot_cross<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let pix = try_opt!(bounds.convert_to_pixel(self.0));

        canvas.draw_line((pix.x + 2.5, pix.y), (pix.x-2.5, pix.y));
        canvas.draw_line((pix.x, pix.y + 2.5), (pix.x, pix.y-2.5));
    }
}

impl Plottable for Point {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        match self.1 {
            PointStyle::Cross => self.plot_cross(bounds, canvas),
            PointStyle::Nothing => {},
        }
    }
}

pub struct Bar(pub GraphCoord);

impl Plottable for Bar {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let GraphCoord { x, y } = self.0;
        let bottom_left = try_opt!(bounds.convert_to_pixel((x - 0.5, 0.0)));
        let top_left = try_opt!(bounds.convert_to_pixel((x - 0.5, y)));
        let bottom_right = try_opt!(bounds.convert_to_pixel((x + 0.5, 0.0)));

        let width = bottom_right.x - bottom_left.x;
        let height = top_left.y - bottom_left.y;

        canvas.fill_rect(bottom_left, width, height);
    }
}

