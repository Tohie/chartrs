use pixel::Color;

/// `PlotStyle` determines whether a `Graph2D` should be plotted
/// as a Bar, Line or Scatter graph
#[derive(Clone, Copy, PartialEq)]
pub enum PlotStyle {
    Bar,
    Line,
    Scatter,  
}

/// `PointStyle` specifies whether markers should be drawn
/// for each point and if so what kind of marker 
#[derive(Clone, Copy, PartialEq)]
pub enum PointStyle {
    Nothing,
    Cross,
    // could have circle, etc.
}

/// `AxisOptions` contains options that are specific to the graph
/// and not a particular series such as any labels, borders or grids
#[derive(Clone, PartialEq)]
pub struct AxisOptions<'a> {
    /// The amount of ticks to display on the x and y axis
    pub tick_count: f64,

    /// The percent of width or height depending on the axis
    /// that the tick should be
    pub tick_size: f64,

    /// The percent of width or height that the x or y 
    /// labels on an axis should be moved away from the axis
    pub label_offset: f64,

    /// Same meaning as label offset however it is for the numbers
    /// on an axis that will be displayed below the tick
    pub number_offset: f64,

    /// A label that will be displayed on the axis
    /// it will be placed halfway between the maximum
    /// value and the origin
    pub label: &'a str,
}

impl <'a> AxisOptions<'a> {
    /// Creates a new AxisOptions using the default values specified
    /// by the implementation of Default by this struct
    pub fn new() -> AxisOptions<'a> {
        AxisOptions::default()
    }

    pub fn tick_count(mut self, tick_count: f64) -> Self {
        self.tick_count = tick_count;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn tick_size(mut self, tick_size: f64) -> Self {
        self.tick_size = tick_size;
        self
    }

    pub fn label_offset(mut self, label_offset: f64) -> Self {
        self.label_offset = label_offset;
        self
    }

    pub fn number_offset(mut self, number_offset: f64) -> Self {
        self.number_offset = number_offset;
        self
    }
}

impl <'a> Default for AxisOptions<'a> {
    fn default() -> AxisOptions<'a> {
        AxisOptions { 
            tick_count: 10.0,
            tick_size: 0.01,
            number_offset: 0.03,
            label_offset: 0.04,
            label: "",
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