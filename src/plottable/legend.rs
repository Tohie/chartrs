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

#[cfg(test)]
mod tests {
    use super::*;
    use canvas::mock_canvas::MockCanvas;
    use canvas::mock_canvas::MockError;
    use graph_dimensions::GraphDimensions;
    use pixel::{GraphCoord, Color};
    use options::DataSetOptions;
    use data_set::DataSet;
    use plottable::Plottable;

    #[test]
    fn test_legend() {
        let mut fake_canvas = MockCanvas::new();
        let mut dims = GraphDimensions::new(600.0, 600.0);
        // Mock is setup so if name is "fail" then it will return an error
        // so we can test the short circuiting logic in Legend
        let fail_opts = DataSetOptions::default().name("fail");
        let pass_opts = DataSetOptions::default().name("pass");

        let data_set_fail = &[&DataSet::from_vecs(vec!(1.0), vec!(2.0), &fail_opts).unwrap()];
        let data_set_pass = &[&DataSet::from_vecs(vec!(1.0), vec!(2.0), &pass_opts).unwrap()];

        dims.max = GraphCoord::new(1.0, 2.0);
        dims.min = GraphCoord::new(1.0, 2.0);

        let legend_fail = Legend(data_set_fail);
        let legend_pass = Legend(data_set_pass);

        assert_eq!(legend_pass.plot(&dims, &mut fake_canvas), Ok(()));
        assert_eq!(legend_fail.plot(&dims, &mut fake_canvas), Err(MockError("write_text failed".to_string())));

        // Legend should change the color, 1, 1, 1 is default color in MockCanvas
        assert!(fake_canvas.color != Color(1, 1, 1));
        // Legend should never clear or show the canvas
        assert_eq!(fake_canvas.shown, 0);
        assert_eq!(fake_canvas.cleared, 0)
    }
}