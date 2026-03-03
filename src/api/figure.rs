use super::axes::Axes2D;
use super::axes3d::Axes3D;

/// Discriminated union for 2D and 3D axes types.
pub(crate) enum AxesType {
    /// Standard 2D plot axes.
    TwoD(Box<Axes2D>),
    /// 3D surface/wireframe axes.
    ThreeD(Box<Axes3D>),
}

/// Top-level figure that owns one or more subplot axes.
pub struct Figure {
    pub(crate) axes: Vec<AxesType>,
    pub(crate) title: Option<String>,
    pub(crate) multiplot: Option<(usize, usize)>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Figure {
    /// Create a new figure with default terminal dimensions (80×24).
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::prelude::*;
    ///
    /// let xs: Vec<f64> = (0..=10).map(|i| i as f64).collect();
    /// let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
    ///
    /// let layout = Layout2D::new()
    ///     .with_title("Example")
    ///     .with_plot(LinePlot::new(xs, ys).with_caption("x²"));
    ///
    /// let output = Figure::new()
    ///     .with_size(60, 15)
    ///     .with_layout(layout)
    ///     .render();
    /// assert!(output.contains("Example"));
    /// ```
    pub fn new() -> Self {
        Self {
            axes: Vec::new(),
            title: None,
            multiplot: None,
            width: 80,
            height: 24,
        }
    }

    /// Set the terminal dimensions (width × height in characters).
    pub fn set_terminal_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the terminal dimensions (consuming builder).
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.set_terminal_size(width, height);
        self
    }

    /// Add a 2D layout to the figure (consuming builder).
    pub fn with_layout(mut self, layout: Axes2D) -> Self {
        self.axes.push(AxesType::TwoD(Box::new(layout)));
        self
    }

    /// Add a 3D layout to the figure (consuming builder).
    pub fn with_layout3d(mut self, layout: Axes3D) -> Self {
        self.axes.push(AxesType::ThreeD(Box::new(layout)));
        self
    }

    /// Set a super-title above all subplots.
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set multiplot layout (rows, cols).
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::prelude::*;
    ///
    /// let xs = vec![0.0, 1.0, 2.0];
    ///
    /// let plot1 = Layout2D::new()
    ///     .with_title("Subplot 1")
    ///     .with_plot(LinePlot::new(xs.iter().copied(), [0.0, 1.0, 0.0]));
    /// let plot2 = Layout2D::new()
    ///     .with_title("Subplot 2")
    ///     .with_plot(LinePlot::new(xs.iter().copied(), [0.0, 2.0, 0.0]));
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(80, 24);
    /// fig.set_multiplot_layout(2, 1);
    /// let output = fig
    ///     .with_layout(plot1)
    ///     .with_layout(plot2)
    ///     .render();
    /// assert!(output.contains("Subplot 1"));
    /// ```
    pub fn set_multiplot_layout(&mut self, rows: usize, cols: usize) -> &mut Self {
        self.multiplot = Some((rows, cols));
        self
    }

    /// Create a new 2D axes subplot and return a mutable reference to it.
    pub fn axes2d(&mut self) -> &mut Axes2D {
        self.axes.push(AxesType::TwoD(Box::new(Axes2D::new())));
        match self.axes.last_mut().unwrap() {
            AxesType::TwoD(ax) => ax,
            _ => unreachable!(),
        }
    }

    /// Create a new 3D axes subplot and return a mutable reference to it.
    pub fn axes3d(&mut self) -> &mut Axes3D {
        self.axes.push(AxesType::ThreeD(Box::new(Axes3D::new())));
        match self.axes.last_mut().unwrap() {
            AxesType::ThreeD(ax) => ax,
            _ => unreachable!(),
        }
    }

    /// Render the figure to a string.
    pub fn render(&self) -> String {
        crate::render::render_figure(self)
    }

    /// Render and print to stdout.
    pub fn show(&self) {
        print!("{}", self.render());
    }

    /// Save the figure to an SVG file.
    ///
    /// * `filepath` - Path to the output file (e.g. "plot.svg").
    /// * `dark_mode` - If true, uses a dark background and defaults to white text.
    pub fn save_svg(&self, filepath: &str, dark_mode: bool) -> std::io::Result<()> {
        let ansi = self.render();
        let svg = crate::export::to_svg(&ansi, dark_mode);
        std::fs::write(filepath, svg)
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
