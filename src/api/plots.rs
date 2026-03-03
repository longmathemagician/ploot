use crate::api::series::SeriesData;
use crate::api::options::{AxisPair, DashType, PlotOption, PointSymbol};
use crate::canvas::color::TermColor;
use crate::api::grid::GridData;

/// A 2D plot or series that can be added to a layout.
pub type Plot2D = SeriesData;

macro_rules! common_plot_options {
    ($name:ident) => {
        impl $name {
            /// Set the legend caption for this plot.
            pub fn with_caption(mut self, caption: &str) -> Self {
                self.options.push(PlotOption::Caption(caption.to_string()));
                self
            }

            /// Set the specific color for this plot (overrides auto-palette).
            pub fn with_color(mut self, color: TermColor) -> Self {
                self.options.push(PlotOption::Color(color));
                self
            }

            /// Choose which axis pair to use.
            pub fn with_axes(mut self, axes: AxisPair) -> Self {
                self.options.push(PlotOption::Axes(axes));
                self
            }
        }
    };
}

/// A line plot connecting data points.
#[derive(Debug, Clone)]
pub struct LinePlot {
    x: Vec<f64>,
    y: Vec<f64>,
    options: Vec<PlotOption>,
}

impl LinePlot {
    /// Create a new line plot.
    pub fn new<Tx, Ty, S>(x: Tx, y: Ty) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            options: Vec::new(),
        }
    }

    /// Set the dash pattern for the line.
    pub fn with_dash(mut self, dash: DashType) -> Self {
        self.options.push(PlotOption::LineStyle(dash));
        self
    }
}
common_plot_options!(LinePlot);

impl From<LinePlot> for Plot2D {
    fn from(plot: LinePlot) -> Self {
        SeriesData::Lines {
            x: plot.x,
            y: plot.y,
            options: plot.options,
        }
    }
}

/// A scatter plot displaying individual point markers.
#[derive(Debug, Clone)]
pub struct ScatterPlot {
    x: Vec<f64>,
    y: Vec<f64>,
    options: Vec<PlotOption>,
}

impl ScatterPlot {
    /// Create a new scatter plot.
    pub fn new<Tx, Ty, S>(x: Tx, y: Ty) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            options: Vec::new(),
        }
    }

    /// Set the point symbol used for markers.
    pub fn with_symbol(mut self, symbol: PointSymbol) -> Self {
        self.options.push(PlotOption::PointSymbol(symbol));
        self
    }
}
common_plot_options!(ScatterPlot);

impl From<ScatterPlot> for Plot2D {
    fn from(plot: ScatterPlot) -> Self {
        SeriesData::Points {
            x: plot.x,
            y: plot.y,
            options: plot.options,
        }
    }
}

/// A plot showing both connected lines and individual point markers.
#[derive(Debug, Clone)]
pub struct LinePointPlot {
    x: Vec<f64>,
    y: Vec<f64>,
    options: Vec<PlotOption>,
}

impl LinePointPlot {
    /// Create a new lines-and-points plot.
    pub fn new<Tx, Ty, S>(x: Tx, y: Ty) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            options: Vec::new(),
        }
    }

    /// Set the dash pattern for the line.
    pub fn with_dash(mut self, dash: DashType) -> Self {
        self.options.push(PlotOption::LineStyle(dash));
        self
    }

    /// Set the point symbol used for markers.
    pub fn with_symbol(mut self, symbol: PointSymbol) -> Self {
        self.options.push(PlotOption::PointSymbol(symbol));
        self
    }
}
common_plot_options!(LinePointPlot);

impl From<LinePointPlot> for Plot2D {
    fn from(plot: LinePointPlot) -> Self {
        SeriesData::LinesPoints {
            x: plot.x,
            y: plot.y,
            options: plot.options,
        }
    }
}

/// A bar chart plot.
#[derive(Debug, Clone)]
pub struct BarPlot {
    x: Vec<f64>,
    y: Vec<f64>,
    options: Vec<PlotOption>,
}

impl BarPlot {
    /// Create a new bar chart from x positions and y heights.
    pub fn new<Tx, Ty, S>(x: Tx, y: Ty) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            options: Vec::new(),
        }
    }

    /// Set the width of each bar (as a fraction of bin spacing).
    pub fn with_box_width(mut self, width: f64) -> Self {
        self.options.push(PlotOption::BoxWidth(width));
        self
    }
}
common_plot_options!(BarPlot);

impl From<BarPlot> for Plot2D {
    fn from(plot: BarPlot) -> Self {
        SeriesData::Boxes {
            x: plot.x,
            y: plot.y,
            options: plot.options,
        }
    }
}

/// A plot rendering a filled area between two functions.
#[derive(Debug, Clone)]
pub struct FillBetweenPlot {
    x: Vec<f64>,
    y1: Vec<f64>,
    y2: Vec<f64>,
    options: Vec<PlotOption>,
}

impl FillBetweenPlot {
    /// Create a new fill-between plot from x values and two y-boundary series.
    pub fn new<Tx, Ty1, Ty2, S>(x: Tx, y1: Ty1, y2: Ty2) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Ty1: IntoIterator<Item = S>,
        Ty2: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            y1: y1.into_iter().map(|v| v.into()).collect(),
            y2: y2.into_iter().map(|v| v.into()).collect(),
            options: Vec::new(),
        }
    }
}
common_plot_options!(FillBetweenPlot);

impl From<FillBetweenPlot> for Plot2D {
    fn from(plot: FillBetweenPlot) -> Self {
        SeriesData::FillBetween {
            x: plot.x,
            y1: plot.y1,
            y2: plot.y2,
            options: plot.options,
        }
    }
}

/// A box-and-whisker plot for displaying statistical distributions.
#[derive(Debug, Clone)]
pub struct BoxAndWhiskerPlot {
    x: Vec<f64>,
    min: Vec<f64>,
    q1: Vec<f64>,
    median: Vec<f64>,
    q3: Vec<f64>,
    max: Vec<f64>,
    options: Vec<PlotOption>,
}

impl BoxAndWhiskerPlot {
    /// Create a new box-and-whisker plot from pre-computed statistics.
    #[allow(clippy::too_many_arguments)]
    pub fn new<Tx, Tmin, Tq1, Tmed, Tq3, Tmax, S>(
        x: Tx,
        min: Tmin,
        q1: Tq1,
        median: Tmed,
        q3: Tq3,
        max: Tmax,
    ) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Tmin: IntoIterator<Item = S>,
        Tq1: IntoIterator<Item = S>,
        Tmed: IntoIterator<Item = S>,
        Tq3: IntoIterator<Item = S>,
        Tmax: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            min: min.into_iter().map(|v| v.into()).collect(),
            q1: q1.into_iter().map(|v| v.into()).collect(),
            median: median.into_iter().map(|v| v.into()).collect(),
            q3: q3.into_iter().map(|v| v.into()).collect(),
            max: max.into_iter().map(|v| v.into()).collect(),
            options: Vec::new(),
        }
    }

    /// Set the width of each box (as a fraction of bin spacing).
    pub fn with_box_width(mut self, width: f64) -> Self {
        self.options.push(PlotOption::BoxWidth(width));
        self
    }
}
common_plot_options!(BoxAndWhiskerPlot);

impl From<BoxAndWhiskerPlot> for Plot2D {
    fn from(plot: BoxAndWhiskerPlot) -> Self {
        SeriesData::BoxAndWhisker {
            x: plot.x,
            min: plot.min,
            q1: plot.q1,
            median: plot.median,
            q3: plot.q3,
            max: plot.max,
            options: plot.options,
        }
    }
}

/// A 2D heatmap plot.
#[derive(Debug, Clone)]
pub struct HeatmapPlot {
    grid: GridData,
    options: Vec<PlotOption>,
}

impl HeatmapPlot {
    /// Create a new heatmap from a [`GridData`] scalar field.
    pub fn new(grid: GridData) -> Self {
        Self {
            grid,
            options: Vec::new(),
        }
    }

    /// Set the colormap globally across the heatmap density.
    pub fn with_colormap(mut self, cmap: crate::canvas::colormap::ColorMapType) -> Self {
        self.options.push(PlotOption::ColorMap(cmap));
        self
    }
}
common_plot_options!(HeatmapPlot);

impl From<HeatmapPlot> for Plot2D {
    fn from(plot: HeatmapPlot) -> Self {
        SeriesData::Heatmap {
            grid: plot.grid,
            options: plot.options,
        }
    }
}

/// A 2D contour plot.
#[derive(Debug, Clone)]
pub struct ContourPlot {
    grid: GridData,
    levels: Option<Vec<f64>>,
    options: Vec<PlotOption>,
}

impl ContourPlot {
    /// Create a new contour plot from a [`GridData`] scalar field.
    pub fn new(grid: GridData) -> Self {
        Self {
            grid,
            levels: None,
            options: Vec::new(),
        }
    }

    /// Set the number of automatically spaced contour levels.
    pub fn with_levels(mut self, levels: usize) -> Self {
        self.options.push(PlotOption::ContourLevels(levels));
        self
    }

    /// Provide explicit contour level values.
    pub fn with_explicit_levels(mut self, levels: &[f64]) -> Self {
        self.levels = Some(levels.to_vec());
        self
    }
}
common_plot_options!(ContourPlot);

impl From<ContourPlot> for Plot2D {
    fn from(plot: ContourPlot) -> Self {
        SeriesData::Contour {
            grid: plot.grid,
            levels: plot.levels,
            options: plot.options,
        }
    }
}

/// A 2D combined heatmap and contour overlay plot.
#[derive(Debug, Clone)]
pub struct HeatmapContourPlot {
    grid: GridData,
    levels: Option<Vec<f64>>,
    options: Vec<PlotOption>,
}

impl HeatmapContourPlot {
    /// Create a new heatmap+contour overlay from a [`GridData`] scalar field.
    pub fn new(grid: GridData) -> Self {
        Self {
            grid,
            levels: None,
            options: Vec::new(),
        }
    }

    /// Set the number of automatically spaced contour levels.
    pub fn with_levels(mut self, levels: usize) -> Self {
        self.options.push(PlotOption::ContourLevels(levels));
        self
    }

    /// Provide explicit contour level values.
    pub fn with_explicit_levels(mut self, levels: &[f64]) -> Self {
        self.levels = Some(levels.to_vec());
        self
    }

    /// Set the colormap globally across the heatmap density.
    pub fn with_colormap(mut self, cmap: crate::canvas::colormap::ColorMapType) -> Self {
        self.options.push(PlotOption::ColorMap(cmap));
        self
    }
}
common_plot_options!(HeatmapContourPlot);

impl From<HeatmapContourPlot> for Plot2D {
    fn from(plot: HeatmapContourPlot) -> Self {
        SeriesData::HeatmapContour {
            grid: plot.grid,
            levels: plot.levels,
            options: plot.options,
        }
    }
}

/// A 1D Histogram plot.
#[derive(Debug, Clone)]
pub struct HistogramPlot {
    data: Vec<f64>,
    bins: usize,
    normalize: bool,
    range: Option<(f64, f64)>,
    options: Vec<PlotOption>,
}

impl HistogramPlot {
    /// Create a new histogram from raw data values and a bin count.
    pub fn new<T, S>(data: T, bins: usize) -> Self
    where
        T: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            data: data.into_iter().map(|v| v.into()).collect(),
            bins,
            normalize: false,
            range: None,
            options: Vec::new(),
        }
    }

    /// Enable or disable probability density normalization.
    pub fn with_normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    /// Restrict binning to a fixed data range.
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.range = Some((min, max));
        self
    }
}
common_plot_options!(HistogramPlot);

impl From<HistogramPlot> for Plot2D {
    fn from(plot: HistogramPlot) -> Self {
        let bins = plot.bins.max(1);
        let min_val = plot.range.map(|r| r.0).unwrap_or_else(|| plot.data.iter().copied().fold(f64::INFINITY, f64::min));
        let max_val = plot.range.map(|r| r.1).unwrap_or_else(|| plot.data.iter().copied().fold(f64::NEG_INFINITY, f64::max));

        if plot.data.is_empty() || min_val >= max_val {
            return SeriesData::Boxes { x: vec![], y: vec![], options: plot.options };
        }

        let bin_width = (max_val - min_val) / bins as f64;
        let mut counts = vec![0.0; bins];
        for &v in &plot.data {
            if v >= min_val && v <= max_val {
                let mut bin_idx = ((v - min_val) / bin_width).floor() as usize;
                if bin_idx >= bins {
                    bin_idx = bins - 1;
                }
                counts[bin_idx] += 1.0;
            }
        }

        if plot.normalize {
            let total = plot.data.len() as f64;
            for c in &mut counts {
                *c /= total * bin_width;
            }
        }

        let bx: Vec<f64> = (0..bins).map(|i| min_val + (i as f64 + 0.5) * bin_width).collect();

        let mut opts = plot.options;
        // set box width to 1.0 so it fills the bin (if user didn't override)
        if !opts.iter().any(|o| matches!(o, PlotOption::BoxWidth(_))) {
            opts.push(PlotOption::BoxWidth(1.0));
        }

        SeriesData::Boxes {
            x: bx,
            y: counts,
            options: opts,
        }
    }
}

/// A 2D Histogram plot (rendered like a heatmap).
#[derive(Debug, Clone)]
pub struct Histogram2DPlot {
    x: Vec<f64>,
    y: Vec<f64>,
    x_bins: usize,
    y_bins: usize,
    options: Vec<PlotOption>,
}

impl Histogram2DPlot {
    /// Create a new 2D histogram from (x, y) samples and bin counts.
    pub fn new<Tx, Ty, S>(x: Tx, y: Ty, x_bins: usize, y_bins: usize) -> Self
    where
        Tx: IntoIterator<Item = S>,
        Ty: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            x_bins,
            y_bins,
            options: Vec::new(),
        }
    }
}
common_plot_options!(Histogram2DPlot);

impl From<Histogram2DPlot> for Plot2D {
    fn from(plot: Histogram2DPlot) -> Self {
        let x_bins = plot.x_bins.max(1);
        let y_bins = plot.y_bins.max(1);

        if plot.x.is_empty() || plot.y.is_empty() {
            let grid = crate::api::grid::GridData::from_fn(|_, _| 0.0, (0.0, 1.0), (0.0, 1.0), 2, 2);
            return SeriesData::Heatmap { grid, options: plot.options };
        }

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for &xv in &plot.x { if xv.is_finite() { x_min = x_min.min(xv); x_max = x_max.max(xv); } }
        for &yv in &plot.y { if yv.is_finite() { y_min = y_min.min(yv); y_max = y_max.max(yv); } }

        if x_max <= x_min || y_max <= y_min {
            let grid = crate::api::grid::GridData::from_fn(|_, _| 0.0, (x_min, x_min+1.0), (y_min, y_min+1.0), 2, 2);
            return SeriesData::Heatmap { grid, options: plot.options };
        }

        let mut z = vec![0.0; x_bins * y_bins];
        let dx = (x_max - x_min) / x_bins as f64;
        let dy = (y_max - y_min) / y_bins as f64;
        let len = plot.x.len().min(plot.y.len());

        for i in 0..len {
            let xv = plot.x[i];
            let yv = plot.y[i];
            if xv >= x_min && xv <= x_max && yv >= y_min && yv <= y_max {
                let mut bx = ((xv - x_min) / dx).floor() as usize;
                let mut by = ((yv - y_min) / dy).floor() as usize;
                if bx >= x_bins { bx = x_bins - 1; }
                if by >= y_bins { by = y_bins - 1; }
                z[by * x_bins + bx] += 1.0;
            }
        }

        let mut z_max: f64 = 0.0;
        for &v in &z { z_max = z_max.max(v); }

        let grid = crate::api::grid::GridData {
            z_values: z,
            nx: x_bins,
            ny: y_bins,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min: 0.0,
            z_max,
        };

        SeriesData::Heatmap {
            grid,
            options: plot.options,
        }
    }
}

/// A Candlestick plot for financial data.
#[derive(Debug, Clone)]
pub struct CandlestickPlot {
    x: Vec<f64>,
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    width: Option<f64>,
    options: Vec<PlotOption>,
}

impl CandlestickPlot {
    /// Create a new candlestick plot from OHLC data.
    pub fn new<Tx, To, Th, Tl, Tc, S>(
        x: Tx,
        open: To,
        high: Th,
        low: Tl,
        close: Tc,
    ) -> Self
    where
        Tx: IntoIterator<Item = S>,
        To: IntoIterator<Item = S>,
        Th: IntoIterator<Item = S>,
        Tl: IntoIterator<Item = S>,
        Tc: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            x: x.into_iter().map(|v| v.into()).collect(),
            open: open.into_iter().map(|v| v.into()).collect(),
            high: high.into_iter().map(|v| v.into()).collect(),
            low: low.into_iter().map(|v| v.into()).collect(),
            close: close.into_iter().map(|v| v.into()).collect(),
            width: None,
            options: Vec::new(),
        }
    }

    /// Set the candlestick body width.
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}
common_plot_options!(CandlestickPlot);

impl From<CandlestickPlot> for Plot2D {
    fn from(plot: CandlestickPlot) -> Self {
        SeriesData::Candlestick {
            x: plot.x,
            open: plot.open,
            high: plot.high,
            low: plot.low,
            close: plot.close,
            width: plot.width,
            options: plot.options,
        }
    }
}

/// A Pie chart plot.
#[derive(Debug, Clone)]
pub struct PiePlot {
    values: Vec<f64>,
    labels: Option<Vec<String>>,
    options: Vec<PlotOption>,
}

impl PiePlot {
    /// Create a new pie chart from slice values.
    pub fn new<T, S>(values: T) -> Self
    where
        T: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        Self {
            values: values.into_iter().map(|v| v.into()).collect(),
            labels: None,
            options: Vec::new(),
        }
    }

    /// Set the labels for each pie slice.
    pub fn with_labels<T, S>(mut self, labels: T) -> Self
    where
        T: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let l: Vec<String> = labels.into_iter().map(|s| s.as_ref().to_string()).collect();
        self.labels = Some(l);
        self
    }
}
common_plot_options!(PiePlot);

impl From<PiePlot> for Plot2D {
    fn from(plot: PiePlot) -> Self {
        SeriesData::Pie {
            values: plot.values,
            labels: plot.labels,
            options: plot.options,
        }
    }
}
