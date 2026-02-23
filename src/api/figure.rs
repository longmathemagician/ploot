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
    /// use ploot::{Figure, PlotOption};
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// {
    ///     let ax = fig.axes2d();
    ///     ax.set_title("Example");
    ///     let xs: Vec<f64> = (0..=10).map(|i| i as f64).collect();
    ///     let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
    ///     ax.lines(
    ///         xs.iter().copied(),
    ///         ys.iter().copied(),
    ///         &[PlotOption::Caption("x²".into())],
    ///     );
    /// }
    /// let output = fig.render();
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

    /// Set the terminal dimensions (character width and height).
    pub fn set_terminal_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;
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
    /// use ploot::Figure;
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(80, 24);
    /// fig.set_multiplot_layout(2, 1); // 2 rows, 1 column
    /// for i in 0..2 {
    ///     let ax = fig.axes2d();
    ///     ax.set_title(&format!("Subplot {}", i + 1));
    ///     let xs = vec![0.0, 1.0, 2.0];
    ///     let ys = vec![0.0, (i + 1) as f64, 0.0];
    ///     ax.lines(xs.iter().copied(), ys.iter().copied(), &[]);
    /// }
    /// let output = fig.render();
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
