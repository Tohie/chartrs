use plottable::Plottable;
use data_set::DataSet;
use graph_dimensions::GraphDimensions;
use pixel::{Pixel, Color};
use canvas::Canvas;

pub struct Legend<'a>(pub &'a [&'a DataSet<'a>]);

impl<'a> Plottable for Legend<'a> {
    fn plot<C: Canvas>(&self, bounds: &GraphDimensions, canvas: &mut C) -> Result<(), C::Err> {
        let Pixel { x: top_x, y: top_y } = 
            bounds.convert_to_pixel((bounds.max.x, bounds.max.y)).expect("bounds.max should be on grid");

        let count = self.0.iter().filter(|&ds| ds.options.name != "").collect::<Vec<_>>().len();

        let row_height = bounds.height * 0.025;
        let title_height = bounds.height * 0.0375;

        let width = bounds.width * 0.15;
        let height = title_height + (row_height * (count as f64));

        let x = top_x - width;
        let y = top_y - height;

        let border = 2.0;

        canvas.set_color(Color(0, 0, 0));
        canvas.fill_rect((x - border, y - border), width + (2.0 * border), height + (2.0 * border))?;

        canvas.set_color(Color(255, 255, 255));
        canvas.fill_rect((x, y), width, height)?;

        let inset_x = 0.05;
        let x = x + (inset_x * width);
        canvas.write_text("Legend", (x, y + (height - title_height)))?;

        let mut y = y + 3.0;
        for &ds in self.0.iter() {
            let half_y = y + (row_height / 2.0);
            
            canvas.set_color(ds.choose_color());
            canvas.draw_line((x, half_y), (x + 20.0, half_y))?;
            
            canvas.set_color(Color(0, 0, 0));
            canvas.write_text(ds.options.name, (x + 25.0, y))?;
            y += row_height;
        }

        Ok(())  
    }
}