use crate::canvas::PALETTE;
use crate::canvas::color::TermColor;

use super::grid::GridData;
use super::options::{
    AutoOption, Coordinate, DashType, LabelOption, LegendOption, Placement, PlotOption, TickOption,
};
use super::series::SeriesData;

/// Configuration for a single axis.
#[derive(Debug, Clone)]
pub struct AxisConfig {
    /// Minimum range value ([`AutoOption::Auto`] for data-driven).
    pub range_min: AutoOption<f64>,
    /// Maximum range value ([`AutoOption::Auto`] for data-driven).
    pub range_max: AutoOption<f64>,
    /// Logarithmic base (e.g. `Some(10.0)`), or `None` for linear.
    pub log_base: Option<f64>,
    /// Whether the axis direction is reversed.
    pub reversed: bool,
    /// Axis label text.
    pub label: Option<String>,
    /// Display options for the axis label.
    pub label_options: Vec<LabelOption>,
    /// Display options for tick marks.
    pub tick_options: Vec<TickOption>,
    /// Custom tick positions and labels, overriding auto-generated ticks.
    pub custom_ticks: Option<Vec<(f64, String)>>,
    /// Whether major grid lines are shown at tick positions.
    pub grid: bool,
    /// Whether minor grid lines are shown between ticks.
    pub minor_grid: bool,
    /// Color for major grid lines.
    pub grid_color: TermColor,
    /// Dash pattern for major grid lines.
    pub grid_dash: DashType,
    /// Color for minor grid lines.
    pub minor_grid_color: TermColor,
    /// Dash pattern for minor grid lines.
    pub minor_grid_dash: DashType,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            range_min: AutoOption::Auto,
            range_max: AutoOption::Auto,
            log_base: None,
            reversed: false,
            label: None,
            label_options: Vec::new(),
            tick_options: Vec::new(),
            custom_ticks: None,
            grid: false,
            minor_grid: false,
            grid_color: TermColor::Default,
            grid_dash: DashType::SmallDot,
            minor_grid_color: TermColor::Default,
            minor_grid_dash: DashType::SmallDot,
        }
    }
}

/// Annotation: text label at a position.
#[derive(Debug, Clone)]
pub struct Annotation {
    /// The annotation text to display.
    pub text: String,
    /// Position as (x, y) [`Coordinate`] pair.
    pub position: (Coordinate, Coordinate),
    /// Display options (color, alignment, etc.).
    pub options: Vec<LabelOption>,
}

/// Arrow between two positions.
#[derive(Debug, Clone)]
pub struct Arrow {
    /// Start position as (x, y) [`Coordinate`] pair.
    pub from: (Coordinate, Coordinate),
    /// End position as (x, y) [`Coordinate`] pair.
    pub to: (Coordinate, Coordinate),
    /// Arrow color.
    pub color: TermColor,
    /// Dash pattern for the arrow shaft.
    pub dash: DashType,
    /// Whether to draw an arrowhead at the `to` end.
    pub head: bool,
}

/// Legend configuration.
#[derive(Debug, Clone)]
pub struct LegendConfig {
    /// Whether the legend is visible.
    pub enabled: bool,
    /// Corner placement within the plot area.
    pub placement: Placement,
    /// Lay entries out horizontally instead of vertically.
    pub horizontal: bool,
    /// Reverse entry order.
    pub reverse: bool,
    /// Optional title shown above legend entries.
    pub title: Option<String>,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            placement: Placement::TopRight,
            horizontal: false,
            reverse: false,
            title: None,
        }
    }
}

/// A 2D axes object that owns series data and axis configuration.
///
/// Follows the gnuplot crate's builder pattern: methods return `&mut Self`.
pub struct Axes2D {
    pub(crate) series: Vec<SeriesData>,
    pub(crate) title: Option<String>,
    pub(crate) x_axis: AxisConfig,
    pub(crate) y_axis: AxisConfig,
    pub(crate) x2_axis: AxisConfig,
    pub(crate) y2_axis: AxisConfig,
    pub(crate) legend: LegendConfig,
    pub(crate) annotations: Vec<Annotation>,
    pub(crate) arrows: Vec<Arrow>,
    /// Palette index for auto-coloring the next series.
    next_color: usize,
}

impl Axes2D {
    pub(crate) fn new() -> Self {
        Self {
            series: Vec::new(),
            title: None,
            x_axis: AxisConfig::default(),
            y_axis: AxisConfig::default(),
            x2_axis: AxisConfig::default(),
            y2_axis: AxisConfig::default(),
            legend: LegendConfig::default(),
            annotations: Vec::new(),
            arrows: Vec::new(),
            next_color: 0,
        }
    }

    /// Returns the auto-assigned color for a new series and advances the counter.
    fn next_auto_color(&mut self) -> TermColor {
        let color = PALETTE[self.next_color % PALETTE.len()];
        self.next_color += 1;
        color
    }

    /// Returns the effective color for a series, checking PlotOption::Color first.
    fn resolve_color(&mut self, options: &[PlotOption]) -> TermColor {
        for opt in options {
            if let PlotOption::Color(c) = opt {
                self.next_color += 1; // still advance to keep palette in sync
                return *c;
            }
        }
        self.next_auto_color()
    }

    // ── Series addition methods ─────────────────────────────────────────

    /// Add a line series.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::{Figure, PlotOption};
    /// use ploot::canvas::color::TermColor;
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// let ax = fig.axes2d();
    /// let xs: Vec<f64> = (0..=10).map(|i| i as f64).collect();
    /// let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
    /// ax.lines(
    ///     xs.iter().copied(),
    ///     ys.iter().copied(),
    ///     &[
    ///         PlotOption::Caption("x²".into()),
    ///         PlotOption::Color(TermColor::Red),
    ///     ],
    /// );
    /// ```
    pub fn lines<Tx, Ty, S>(&mut self, x: Tx, y: Ty, options: &[PlotOption]) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::Lines {
            x,
            y,
            options: options.to_vec(),
        });
        self
    }

    /// Add a point (scatter) series.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::{Figure, PlotOption, PointSymbol};
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// let ax = fig.axes2d();
    /// let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ys = vec![1.0, 4.0, 2.0, 5.0, 3.0];
    /// ax.points(
    ///     xs.iter().copied(),
    ///     ys.iter().copied(),
    ///     &[PlotOption::PointSymbol(PointSymbol::Cross)],
    /// );
    /// ```
    pub fn points<Tx, Ty, S>(&mut self, x: Tx, y: Ty, options: &[PlotOption]) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::Points {
            x,
            y,
            options: options.to_vec(),
        });
        self
    }

    /// Add a lines+points series.
    pub fn lines_points<Tx, Ty, S>(&mut self, x: Tx, y: Ty, options: &[PlotOption]) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::LinesPoints {
            x,
            y,
            options: options.to_vec(),
        });
        self
    }

    /// Add a box (bar chart) series.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::{Figure, PlotOption};
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// let ax = fig.axes2d();
    /// let xs = vec![1.0, 2.0, 3.0];
    /// let ys = vec![5.0, 3.0, 7.0];
    /// ax.boxes(
    ///     xs.iter().copied(),
    ///     ys.iter().copied(),
    ///     &[PlotOption::Caption("Sales".into())],
    /// );
    /// ```
    pub fn boxes<Tx, Ty, S>(&mut self, x: Tx, y: Ty, options: &[PlotOption]) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::Boxes {
            x,
            y,
            options: options.to_vec(),
        });
        self
    }

    /// Add a fill-between series (area between two y curves).
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::Figure;
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// let ax = fig.axes2d();
    /// let xs: Vec<f64> = (0..=10).map(|i| i as f64).collect();
    /// let y_lo: Vec<f64> = xs.iter().map(|x| x * 0.5).collect();
    /// let y_hi: Vec<f64> = xs.iter().map(|x| x * 1.5).collect();
    /// ax.fill_between(
    ///     xs.iter().copied(),
    ///     y_lo.iter().copied(),
    ///     y_hi.iter().copied(),
    ///     &[],
    /// );
    /// ```
    pub fn fill_between<Tx, Ty1, Ty2, S>(
        &mut self,
        x: Tx,
        y1: Ty1,
        y2: Ty2,
        options: &[PlotOption],
    ) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty1: IntoIterator<Item = S>,
        Ty2: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y1: Vec<f64> = y1.into_iter().map(|v| v.into()).collect();
        let y2: Vec<f64> = y2.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::FillBetween {
            x,
            y1,
            y2,
            options: options.to_vec(),
        });
        self
    }

    /// Add a y-error-bar series.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::Figure;
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// let ax = fig.axes2d();
    /// let xs = vec![1.0, 2.0, 3.0];
    /// let ys = vec![10.0, 20.0, 15.0];
    /// let y_lo = vec![8.0, 17.0, 12.0];
    /// let y_hi = vec![12.0, 23.0, 18.0];
    /// ax.y_error_bars(
    ///     xs.iter().copied(),
    ///     ys.iter().copied(),
    ///     y_lo.iter().copied(),
    ///     y_hi.iter().copied(),
    ///     &[],
    /// );
    /// ```
    pub fn y_error_bars<Tx, Ty, Tyl, Tyh, S>(
        &mut self,
        x: Tx,
        y: Ty,
        y_low: Tyl,
        y_high: Tyh,
        options: &[PlotOption],
    ) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        Tyl: IntoIterator<Item = S>,
        Tyh: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let y_low: Vec<f64> = y_low.into_iter().map(|v| v.into()).collect();
        let y_high: Vec<f64> = y_high.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::YErrorBars {
            x,
            y,
            y_low,
            y_high,
            options: options.to_vec(),
        });
        self
    }

    /// Add an x-error-bar series.
    pub fn x_error_bars<Tx, Ty, Txl, Txh, S>(
        &mut self,
        x: Tx,
        y: Ty,
        x_low: Txl,
        x_high: Txh,
        options: &[PlotOption],
    ) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        Txl: IntoIterator<Item = S>,
        Txh: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let x_low: Vec<f64> = x_low.into_iter().map(|v| v.into()).collect();
        let x_high: Vec<f64> = x_high.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::XErrorBars {
            x,
            y,
            x_low,
            x_high,
            options: options.to_vec(),
        });
        self
    }

    /// Add a y-error-lines series (error bars + connecting lines).
    pub fn y_error_lines<Tx, Ty, Tyl, Tyh, S>(
        &mut self,
        x: Tx,
        y: Ty,
        y_low: Tyl,
        y_high: Tyh,
        options: &[PlotOption],
    ) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        Tyl: IntoIterator<Item = S>,
        Tyh: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let y_low: Vec<f64> = y_low.into_iter().map(|v| v.into()).collect();
        let y_high: Vec<f64> = y_high.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::YErrorLines {
            x,
            y,
            y_low,
            y_high,
            options: options.to_vec(),
        });
        self
    }

    /// Add an x-error-lines series (error bars + connecting lines).
    pub fn x_error_lines<Tx, Ty, Txl, Txh, S>(
        &mut self,
        x: Tx,
        y: Ty,
        x_low: Txl,
        x_high: Txh,
        options: &[PlotOption],
    ) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        Txl: IntoIterator<Item = S>,
        Txh: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let y: Vec<f64> = y.into_iter().map(|v| v.into()).collect();
        let x_low: Vec<f64> = x_low.into_iter().map(|v| v.into()).collect();
        let x_high: Vec<f64> = x_high.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::XErrorLines {
            x,
            y,
            x_low,
            x_high,
            options: options.to_vec(),
        });
        self
    }

    /// Add a box-and-whisker series.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::Figure;
    ///
    /// let mut fig = Figure::new();
    /// fig.set_terminal_size(60, 15);
    /// let ax = fig.axes2d();
    /// let xs   = vec![1.0, 2.0, 3.0];
    /// let mins = vec![1.0, 2.0, 0.5];
    /// let q1s  = vec![3.0, 4.0, 2.0];
    /// let meds = vec![5.0, 6.0, 4.0];
    /// let q3s  = vec![7.0, 8.0, 6.0];
    /// let maxs = vec![9.0, 10.0, 8.0];
    /// ax.box_and_whisker(
    ///     xs.iter().copied(),
    ///     mins.iter().copied(),
    ///     q1s.iter().copied(),
    ///     meds.iter().copied(),
    ///     q3s.iter().copied(),
    ///     maxs.iter().copied(),
    ///     &[],
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn box_and_whisker<Tx, Tmin, Tq1, Tmed, Tq3, Tmax, S>(
        &mut self,
        x: Tx,
        min: Tmin,
        q1: Tq1,
        median: Tmed,
        q3: Tq3,
        max: Tmax,
        options: &[PlotOption],
    ) -> &mut Self
    where
        Tx: IntoIterator<Item = S>,
        Tmin: IntoIterator<Item = S>,
        Tq1: IntoIterator<Item = S>,
        Tmed: IntoIterator<Item = S>,
        Tq3: IntoIterator<Item = S>,
        Tmax: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let x: Vec<f64> = x.into_iter().map(|v| v.into()).collect();
        let min: Vec<f64> = min.into_iter().map(|v| v.into()).collect();
        let q1: Vec<f64> = q1.into_iter().map(|v| v.into()).collect();
        let median: Vec<f64> = median.into_iter().map(|v| v.into()).collect();
        let q3: Vec<f64> = q3.into_iter().map(|v| v.into()).collect();
        let max: Vec<f64> = max.into_iter().map(|v| v.into()).collect();
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::BoxAndWhisker {
            x,
            min,
            q1,
            median,
            q3,
            max,
            options: options.to_vec(),
        });
        self
    }

    /// Add a heatmap series from grid data.
    pub fn heatmap(&mut self, grid: GridData, options: &[PlotOption]) -> &mut Self {
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::Heatmap {
            grid,
            options: options.to_vec(),
        });
        self
    }

    /// Add a contour series from grid data.
    pub fn contour(
        &mut self,
        grid: GridData,
        levels: Option<&[f64]>,
        options: &[PlotOption],
    ) -> &mut Self {
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::Contour {
            grid,
            levels: levels.map(|l| l.to_vec()),
            options: options.to_vec(),
        });
        self
    }

    /// Add a combined heatmap + contour overlay.
    pub fn heatmap_contour(
        &mut self,
        grid: GridData,
        levels: Option<&[f64]>,
        options: &[PlotOption],
    ) -> &mut Self {
        let _color = self.resolve_color(options);
        self.series.push(SeriesData::HeatmapContour {
            grid,
            levels: levels.map(|l| l.to_vec()),
            options: options.to_vec(),
        });
        self
    }

    // ── Axis configuration (AxesCommon trait methods) ────────────────────

    /// Set the plot title.
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set x-axis range.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::{AutoOption, Figure};
    ///
    /// let mut fig = Figure::new();
    /// let ax = fig.axes2d();
    /// ax.set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(100.0));
    /// ```
    pub fn set_x_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self {
        self.x_axis.range_min = min;
        self.x_axis.range_max = max;
        self
    }

    /// Set y-axis range.
    pub fn set_y_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self {
        self.y_axis.range_min = min;
        self.y_axis.range_max = max;
        self
    }

    /// Set secondary x-axis range.
    pub fn set_x2_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self {
        self.x2_axis.range_min = min;
        self.x2_axis.range_max = max;
        self
    }

    /// Set secondary y-axis range.
    pub fn set_y2_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self {
        self.y2_axis.range_min = min;
        self.y2_axis.range_max = max;
        self
    }

    /// Set x-axis label.
    pub fn set_x_label(&mut self, label: &str, options: &[LabelOption]) -> &mut Self {
        self.x_axis.label = Some(label.to_string());
        self.x_axis.label_options = options.to_vec();
        self
    }

    /// Set y-axis label.
    pub fn set_y_label(&mut self, label: &str, options: &[LabelOption]) -> &mut Self {
        self.y_axis.label = Some(label.to_string());
        self.y_axis.label_options = options.to_vec();
        self
    }

    /// Set secondary x-axis label.
    pub fn set_x2_label(&mut self, label: &str, options: &[LabelOption]) -> &mut Self {
        self.x2_axis.label = Some(label.to_string());
        self.x2_axis.label_options = options.to_vec();
        self
    }

    /// Set secondary y-axis label.
    pub fn set_y2_label(&mut self, label: &str, options: &[LabelOption]) -> &mut Self {
        self.y2_axis.label = Some(label.to_string());
        self.y2_axis.label_options = options.to_vec();
        self
    }

    /// Set x-axis tick options.
    pub fn set_x_ticks(&mut self, options: &[TickOption]) -> &mut Self {
        self.x_axis.tick_options = options.to_vec();
        self
    }

    /// Set y-axis tick options.
    pub fn set_y_ticks(&mut self, options: &[TickOption]) -> &mut Self {
        self.y_axis.tick_options = options.to_vec();
        self
    }

    /// Set custom x-axis tick positions and labels.
    pub fn set_x_ticks_custom(&mut self, ticks: &[(f64, &str)]) -> &mut Self {
        self.x_axis.custom_ticks = Some(ticks.iter().map(|(v, l)| (*v, l.to_string())).collect());
        self
    }

    /// Set custom y-axis tick positions and labels.
    pub fn set_y_ticks_custom(&mut self, ticks: &[(f64, &str)]) -> &mut Self {
        self.y_axis.custom_ticks = Some(ticks.iter().map(|(v, l)| (*v, l.to_string())).collect());
        self
    }

    /// Enable x-axis grid lines at major tick positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::Figure;
    ///
    /// let mut fig = Figure::new();
    /// let ax = fig.axes2d();
    /// ax.set_x_grid(true);
    /// ax.set_y_grid(true);
    /// ```
    pub fn set_x_grid(&mut self, enabled: bool) -> &mut Self {
        self.x_axis.grid = enabled;
        self
    }

    /// Enable y-axis grid lines at major tick positions.
    pub fn set_y_grid(&mut self, enabled: bool) -> &mut Self {
        self.y_axis.grid = enabled;
        self
    }

    /// Enable x-axis minor grid lines.
    pub fn set_x_minor_grid(&mut self, enabled: bool) -> &mut Self {
        self.x_axis.minor_grid = enabled;
        self
    }

    /// Enable y-axis minor grid lines.
    pub fn set_y_minor_grid(&mut self, enabled: bool) -> &mut Self {
        self.y_axis.minor_grid = enabled;
        self
    }

    /// Set x-axis log scale.
    ///
    /// Pass `Some(10.0)` for base-10 logarithmic scaling, or `None` to
    /// revert to linear.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::Figure;
    ///
    /// let mut fig = Figure::new();
    /// let ax = fig.axes2d();
    /// ax.set_x_log(Some(10.0));
    /// ```
    pub fn set_x_log(&mut self, base: Option<f64>) -> &mut Self {
        self.x_axis.log_base = base;
        self
    }

    /// Set y-axis log scale.
    pub fn set_y_log(&mut self, base: Option<f64>) -> &mut Self {
        self.y_axis.log_base = base;
        self
    }

    /// Set secondary y-axis log scale.
    pub fn set_y2_log(&mut self, base: Option<f64>) -> &mut Self {
        self.y2_axis.log_base = base;
        self
    }

    /// Set secondary x-axis log scale.
    pub fn set_x2_log(&mut self, base: Option<f64>) -> &mut Self {
        self.x2_axis.log_base = base;
        self
    }

    /// Reverse x-axis direction.
    pub fn set_x_reverse(&mut self) -> &mut Self {
        self.x_axis.reversed = true;
        self
    }

    /// Reverse y-axis direction.
    pub fn set_y_reverse(&mut self) -> &mut Self {
        self.y_axis.reversed = true;
        self
    }

    /// Configure legend display.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::{Figure, LegendOption, Placement};
    ///
    /// let mut fig = Figure::new();
    /// let ax = fig.axes2d();
    /// ax.set_legend(&[
    ///     LegendOption::Placement(Placement::TopLeft),
    ///     LegendOption::Title("Legend".into()),
    /// ]);
    /// ```
    pub fn set_legend(&mut self, options: &[LegendOption]) -> &mut Self {
        for opt in options {
            match opt {
                LegendOption::Reverse => self.legend.reverse = true,
                LegendOption::Horizontal => self.legend.horizontal = true,
                LegendOption::Placement(p) => self.legend.placement = *p,
                LegendOption::Title(t) => self.legend.title = Some(t.clone()),
            }
        }
        self
    }

    /// Disable the legend.
    pub fn hide_legend(&mut self) -> &mut Self {
        self.legend.enabled = false;
        self
    }

    /// Add a text annotation at a position.
    pub fn label(
        &mut self,
        text: &str,
        x: Coordinate,
        y: Coordinate,
        options: &[LabelOption],
    ) -> &mut Self {
        self.annotations.push(Annotation {
            text: text.to_string(),
            position: (x, y),
            options: options.to_vec(),
        });
        self
    }

    /// Add an arrow between two positions.
    pub fn arrow(
        &mut self,
        from_x: Coordinate,
        from_y: Coordinate,
        to_x: Coordinate,
        to_y: Coordinate,
    ) -> &mut Self {
        self.arrows.push(Arrow {
            from: (from_x, from_y),
            to: (to_x, to_y),
            color: TermColor::Default,
            dash: DashType::Solid,
            head: true,
        });
        self
    }

    /// Set grid line color and dash.
    pub fn set_grid_options(&mut self, color: TermColor, dash: DashType) -> &mut Self {
        self.x_axis.grid_color = color;
        self.x_axis.grid_dash = dash;
        self.y_axis.grid_color = color;
        self.y_axis.grid_dash = dash;
        self
    }

    /// Set minor grid line color and dash.
    pub fn set_minor_grid_options(&mut self, color: TermColor, dash: DashType) -> &mut Self {
        self.x_axis.minor_grid_color = color;
        self.x_axis.minor_grid_dash = dash;
        self.y_axis.minor_grid_color = color;
        self.y_axis.minor_grid_dash = dash;
        self
    }
}
