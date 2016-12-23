use options::AxisOptions;
use graph_bounds::GraphBounds;
use canvas::Canvas;
use plottable::Plottable;
use pixel::Color;

#[derive(Copy, Clone, PartialEq)]
pub enum AxisKind {
    X, Y,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Axis<'a> {
    kind: AxisKind,
    tick_amount: f64,
    max: f64,
    min: f64,
    space: f64,

    opts: &'a AxisOptions<'a>,
}

impl<'a> Axis<'a> {
    pub fn new(kind: AxisKind, tick_amount: f64, max: f64, min: f64, space: f64, opts: &'a AxisOptions) -> Axis<'a> {
        Axis { 
            kind: kind,
            tick_amount: tick_amount,
            max: max,
            min: min,
            space: space,
            opts: opts,
        }
    }

    fn draw_axis<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
        let (is_x, is_y) = match self.kind {
            AxisKind::X => (1.0, 0.0),
            AxisKind::Y => (0.0, 1.0),
        };

        let min = bounds.convert_to_pixel((self.min * is_x, self.min * is_y));
        let max = bounds.convert_to_pixel((self.max * is_x, self.max * is_y));
        
        canvas.draw_line(min, max);

        let mut i = self.min;
        while i <= self.max {
            let pix = bounds.convert_to_pixel((i*is_x, i*is_y));
            
            let tick_size = self.space * self.opts.tick_size;
            canvas.draw_line((pix.x, pix.y), (pix.x - (tick_size*is_y), pix.y - (tick_size*is_x)));

            let number_offset = self.space * self.opts.number_offset;
            canvas.write_num(i, (pix.x - (number_offset * is_y), pix.y - (number_offset * is_x)));

            i += self.tick_amount;
        }
    }

    fn write_label<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
        let (is_x, is_y) = match self.kind {
            AxisKind::X => (1.0, 0.0),
            AxisKind::Y => (0.0, 1.0),
        };

        let x = self.max / 2.0;
        let y = self.max / 2.0;
        let pix = bounds.convert_to_pixel((x * is_x, y * is_y));

        let label_offset = self.space * self.opts.label_offset;

        let x_offset = label_offset * is_y;
        let y_offset = label_offset * is_x;

        canvas.write_text(self.opts.label, (pix.x - x_offset, pix.y - y_offset));
    }
}

impl<'a> Plottable for Axis<'a> {
    fn plot<C: Canvas>(&self, bounds: &GraphBounds, canvas: &mut C) {
        canvas.set_color(Color(0, 0, 0));
        self.draw_axis(bounds, canvas);
        self.write_label(bounds, canvas);
    }
}