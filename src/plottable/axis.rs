use options::AxisOptions;
use graph_dimensions::GraphDimensions;
use canvas::Canvas;
use plottable::Plottable;
use labeller::{Labeller, Label};
use pixel::{Color, Pixel};

#[derive(Copy, Clone, PartialEq)]
pub struct Axis<'a> {
    x_opts: &'a AxisOptions<'a>,
    y_opts: &'a AxisOptions<'a>,

    x_label: Label,
    y_label: Label,
}

impl<'a> Axis<'a> {
    pub fn new(x_label: Label, y_label: Label, x_opts: &'a AxisOptions<'a>, y_opts: &'a AxisOptions<'a>) -> Axis<'a> {
        Axis {
            x_opts: x_opts,
            y_opts: y_opts,
            x_label: x_label,
            y_label: y_label,
        }
    }

    pub fn from_dimensions_mut(dimensions: &mut GraphDimensions, 
        x_opts: &'a AxisOptions<'a>, y_opts: &'a AxisOptions<'a>) -> Axis<'a> {

        let axis = Axis::from_dimensions(dimensions, x_opts, y_opts);
        dimensions.adjust((axis.x_label.max, axis.y_label.max), (axis.x_label.min, axis.y_label.min));
        axis
    }

    pub fn from_dimensions(dimensions: &GraphDimensions,
        x_opts: &'a AxisOptions<'a>, y_opts: &'a AxisOptions<'a>) -> Axis<'a> {

        let labeller = Labeller::in_base10();

        let GraphDimensions { max, min, .. } = *dimensions;
        let x_label = labeller.search(min.x, max.x, x_opts.tick_count as i32);
        let y_label = labeller.search(min.y, max.y, y_opts.tick_count as i32);

        Axis::new(x_label, y_label, x_opts, y_opts)
    }

    fn draw_axis<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let bottom_left = bounds.convert_to_pixel((bounds.min.x, bounds.min.y));
        let top_left = bounds.convert_to_pixel((bounds.min.x, bounds.max.y));
        let bottom_right = bounds.convert_to_pixel((bounds.max.x, bounds.min.y));
        let top_right = bounds.convert_to_pixel((bounds.max.x, bounds.max.y));

        canvas.draw_line(bottom_left, bottom_right);
        canvas.draw_line(bottom_left, top_left);
        canvas.draw_line(top_left, top_right);
        canvas.draw_line(bottom_right, top_right);

        let mut x = bounds.min.x;
        while x <= bounds.max.x {
            let pix = bounds.convert_to_pixel((x, bounds.min.y));
            
            let tick_size = bounds.width * self.x_opts.tick_size;
            canvas.draw_line(pix, Pixel::new(pix.x, pix.y - tick_size));

            let number_offset = bounds.width * self.x_opts.number_offset;
            canvas.write_num_centred(x, (pix.x, pix.y - number_offset));

            let top = bounds.convert_to_pixel((x, bounds.max.y));
            canvas.draw_line(pix, top);

            x += self.x_label.step;
        }

        let mut y = bounds.min.y;
        while y <= bounds.max.y {
            let pix = bounds.convert_to_pixel((bounds.min.x, y));
            
            let tick_size = bounds.height * self.y_opts.tick_size;
            canvas.draw_line((pix.x, pix.y), (pix.x - tick_size, pix.y));

            let number_offset = bounds.height * self.y_opts.number_offset;
            canvas.write_num_centred(y, (pix.x - number_offset, pix.y));

            let right = bounds.convert_to_pixel((bounds.max.x, y));
            canvas.draw_line(pix, right);

            y += self.y_label.step;
        }
    }

    fn write_label<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        let x = bounds.width / 2.0;
        let y = bounds.height / 2.0;

        let origin = bounds.convert_to_pixel((bounds.min.x, bounds.min.y));
        let x_offset = bounds.width * self.x_opts.label_offset;
        let y_offset = bounds.height * self.y_opts.label_offset;

        canvas.write_text_centred(self.x_opts.label, (x, origin.y - y_offset));
        canvas.write_text_centred(self.y_opts.label, (origin.x - x_offset, y));
    }

}

impl<'a> Plottable for Axis<'a> {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) {
        canvas.set_color(Color(0, 0, 0));
        self.draw_axis(bounds, canvas);
        self.write_label(bounds, canvas);
    }
}