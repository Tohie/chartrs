mod graph_2d;
mod data_set;
mod axis_2d;

pub use self::data_set::DataSet;
pub use self::graph_2d::Graph2D;

use pixel::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum PlotStyle {
    Bar,
    Line,
    Scatter,  
}

#[derive(Clone, Copy, PartialEq)]
pub enum PointStyle {
    Nothing,
    Cross,
    // could have circle, etc.
}

#[derive(Clone, PartialEq)]
pub struct PlotOptions<'a> {
    pub tick_count: f64,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub title: &'a str,
    pub horizontal_border: f64,
    pub vertical_border: f64,
}

impl <'a> PlotOptions<'a> {
    pub fn new() -> PlotOptions<'a> {
        PlotOptions::default()
    }

    pub fn tick_count(mut self, tick_count: f64) -> Self {
        self.tick_count = tick_count;
        self
    }

    pub fn horizontal_border(mut self, horizontal_border: f64) -> Self {
        self.horizontal_border = horizontal_border;
        self
    }

    pub fn vertical_border(mut self, vertical_border: f64) -> Self {
        self.vertical_border = vertical_border;
        self
    }

    pub fn x_label(mut self, x_label: &'a str) -> Self {
        self.x_label = x_label;
        self
    }

    pub fn y_label(mut self, y_label: &'a str) -> Self {
        self.y_label = y_label;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
}

impl <'a> Default for PlotOptions<'a> {
    fn default() -> PlotOptions<'a> {
        PlotOptions { 
            tick_count: 10.0,
            x_label: "",
            y_label: "",
            title: "",
            horizontal_border: 0.1,
            vertical_border: 0.1,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DataSetOptions<'a> {
    pub plot_style: PlotStyle,  
    pub point_style: PointStyle,
    pub color: Color,
    pub random_color: bool,
    pub colors: Option<&'a [Color]>,
}

impl <'a> DataSetOptions<'a> {
    pub fn new() -> DataSetOptions<'a> {
        DataSetOptions::default()
    }
    
    pub fn plot_style(mut self, plot_style: PlotStyle) -> Self {
        self.plot_style = plot_style;
        self
    }

    pub fn point_style(mut self, point_style: PointStyle) -> Self {
        self.point_style = point_style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn random_color(mut self, b: bool) -> Self {
        self.random_color = b;
        self
    }

    pub fn colors<C>(mut self, colors: C) -> Self 
        where C: Into<Option<&'a [Color]>> {

        self.colors = colors.into();
        self
    }
}

impl <'a> Default for DataSetOptions<'a> {
    fn default() -> DataSetOptions<'a> {
        DataSetOptions { 
            plot_style: PlotStyle::Line,
            point_style: PointStyle::Nothing,
            color: Color(0, 0, 0),
            random_color: false,
            colors: None,
        }
    }
}

pub trait Graph {
    fn plot(&mut self);
}
