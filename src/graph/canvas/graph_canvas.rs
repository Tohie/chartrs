use graph::canvas::GraphBounds;
use graph::plottable::Plottable;
use canvas::Canvas;
use pixel::Color;

pub struct GraphCanvas<'a, T: 'a> {
    bounds: GraphBounds,

    canvas: &'a mut T
}

impl<'a, T: Canvas> GraphCanvas<'a, T> {
    pub fn new(bounds: GraphBounds, canvas: &'a mut T) -> GraphCanvas<'a, T> {
        GraphCanvas {
            bounds: bounds,
            
            canvas: canvas,
        }
    }

    pub fn plot<P: Plottable>(&mut self, p: &P) {
        p.plot(&self.bounds, self.canvas)
    }

    pub fn show(&mut self) {
        self.canvas.show();
    }

    pub fn set_color<C: Into<Color>>(&mut self, c: C) {
        self.canvas.set_color(c)
    }
}
