use super::grid::GridData;
use super::options::{AxisPair, PlotOption};

/// Data and configuration for a single plot series.
#[derive(Debug, Clone)]
pub enum SeriesData {
    /// Connected line segments between data points.
    Lines {
        /// X coordinates.
        x: Vec<f64>,
        /// Y coordinates.
        y: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Scatter plot — individual markers at each data point.
    Points {
        /// X coordinates.
        x: Vec<f64>,
        /// Y coordinates.
        y: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Lines with point markers at each data point.
    LinesPoints {
        /// X coordinates.
        x: Vec<f64>,
        /// Y coordinates.
        y: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Vertical bars (bar chart) from baseline to each y value.
    Boxes {
        /// X coordinates (bar centers).
        x: Vec<f64>,
        /// Y coordinates (bar heights).
        y: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Filled area between two y curves.
    FillBetween {
        /// X coordinates (shared by both curves).
        x: Vec<f64>,
        /// Lower y boundary.
        y1: Vec<f64>,
        /// Upper y boundary.
        y2: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Vertical error bars at each data point.
    YErrorBars {
        /// X coordinates.
        x: Vec<f64>,
        /// Y center values.
        y: Vec<f64>,
        /// Lower error bound.
        y_low: Vec<f64>,
        /// Upper error bound.
        y_high: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Horizontal error bars at each data point.
    XErrorBars {
        /// X center values.
        x: Vec<f64>,
        /// Y coordinates.
        y: Vec<f64>,
        /// Lower error bound.
        x_low: Vec<f64>,
        /// Upper error bound.
        x_high: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Vertical error bars with connecting lines through center points.
    YErrorLines {
        /// X coordinates.
        x: Vec<f64>,
        /// Y center values.
        y: Vec<f64>,
        /// Lower error bound.
        y_low: Vec<f64>,
        /// Upper error bound.
        y_high: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Horizontal error bars with connecting lines through center points.
    XErrorLines {
        /// X center values.
        x: Vec<f64>,
        /// Y coordinates.
        y: Vec<f64>,
        /// Lower error bound.
        x_low: Vec<f64>,
        /// Upper error bound.
        x_high: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Box-and-whisker plot showing statistical distribution.
    BoxAndWhisker {
        /// X positions for each box.
        x: Vec<f64>,
        /// Minimum (whisker low).
        min: Vec<f64>,
        /// First quartile (box low).
        q1: Vec<f64>,
        /// Median line.
        median: Vec<f64>,
        /// Third quartile (box high).
        q3: Vec<f64>,
        /// Maximum (whisker high).
        max: Vec<f64>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Heatmap — 2D grid rendered with color-mapped density.
    Heatmap {
        /// Grid data.
        grid: GridData,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Contour — isolines extracted from a 2D grid.
    Contour {
        /// Grid data.
        grid: GridData,
        /// Optional explicit contour levels.
        levels: Option<Vec<f64>>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
    /// Combined heatmap + contour overlay.
    HeatmapContour {
        /// Grid data.
        grid: GridData,
        /// Optional explicit contour levels.
        levels: Option<Vec<f64>>,
        /// Per-series display options.
        options: Vec<PlotOption>,
    },
}

impl SeriesData {
    /// Returns a reference to the options slice for this series.
    pub fn options(&self) -> &[PlotOption] {
        match self {
            SeriesData::Lines { options, .. }
            | SeriesData::Points { options, .. }
            | SeriesData::LinesPoints { options, .. }
            | SeriesData::Boxes { options, .. }
            | SeriesData::FillBetween { options, .. }
            | SeriesData::YErrorBars { options, .. }
            | SeriesData::XErrorBars { options, .. }
            | SeriesData::YErrorLines { options, .. }
            | SeriesData::XErrorLines { options, .. }
            | SeriesData::BoxAndWhisker { options, .. }
            | SeriesData::Heatmap { options, .. }
            | SeriesData::Contour { options, .. }
            | SeriesData::HeatmapContour { options, .. } => options,
        }
    }

    /// Returns the axis pair this series is plotted against.
    pub fn axis_pair(&self) -> AxisPair {
        self.options()
            .iter()
            .find_map(|o| {
                if let PlotOption::Axes(a) = o {
                    Some(*a)
                } else {
                    None
                }
            })
            .unwrap_or(AxisPair::X1Y1)
    }

    /// Returns the x data for this series (empty for grid-based types).
    pub fn x_data(&self) -> &[f64] {
        match self {
            SeriesData::Lines { x, .. }
            | SeriesData::Points { x, .. }
            | SeriesData::LinesPoints { x, .. }
            | SeriesData::Boxes { x, .. }
            | SeriesData::FillBetween { x, .. }
            | SeriesData::YErrorBars { x, .. }
            | SeriesData::XErrorBars { x, .. }
            | SeriesData::YErrorLines { x, .. }
            | SeriesData::XErrorLines { x, .. }
            | SeriesData::BoxAndWhisker { x, .. } => x,
            SeriesData::Heatmap { .. }
            | SeriesData::Contour { .. }
            | SeriesData::HeatmapContour { .. } => &[],
        }
    }

    /// Returns the primary y data for this series (empty for grid-based types).
    pub fn y_data(&self) -> &[f64] {
        match self {
            SeriesData::Lines { y, .. }
            | SeriesData::Points { y, .. }
            | SeriesData::LinesPoints { y, .. }
            | SeriesData::Boxes { y, .. }
            | SeriesData::YErrorBars { y, .. }
            | SeriesData::XErrorBars { y, .. }
            | SeriesData::YErrorLines { y, .. }
            | SeriesData::XErrorLines { y, .. } => y,
            SeriesData::FillBetween { y1, .. } => y1,
            SeriesData::BoxAndWhisker { median, .. } => median,
            SeriesData::Heatmap { .. }
            | SeriesData::Contour { .. }
            | SeriesData::HeatmapContour { .. } => &[],
        }
    }

    /// Collects all y-values that affect the data range for this series.
    pub fn y_range_values(&self) -> Vec<f64> {
        match self {
            SeriesData::Lines { y, .. }
            | SeriesData::Points { y, .. }
            | SeriesData::LinesPoints { y, .. }
            | SeriesData::Boxes { y, .. } => y.clone(),
            SeriesData::FillBetween { y1, y2, .. } => {
                let mut v = y1.clone();
                v.extend_from_slice(y2);
                v
            }
            SeriesData::YErrorBars { y_low, y_high, .. }
            | SeriesData::YErrorLines { y_low, y_high, .. } => {
                let mut v = y_low.clone();
                v.extend_from_slice(y_high);
                v
            }
            SeriesData::XErrorBars { y, .. } | SeriesData::XErrorLines { y, .. } => y.clone(),
            SeriesData::BoxAndWhisker { min, max, .. } => {
                let mut v = min.clone();
                v.extend_from_slice(max);
                v
            }
            SeriesData::Heatmap { .. }
            | SeriesData::Contour { .. }
            | SeriesData::HeatmapContour { .. } => Vec::new(),
        }
    }

    /// Collects all x-values that affect the data range for this series.
    pub fn x_range_values(&self) -> Vec<f64> {
        match self {
            SeriesData::XErrorBars { x_low, x_high, .. }
            | SeriesData::XErrorLines { x_low, x_high, .. } => {
                let mut v = x_low.clone();
                v.extend_from_slice(x_high);
                v
            }
            SeriesData::Heatmap { .. }
            | SeriesData::Contour { .. }
            | SeriesData::HeatmapContour { .. } => Vec::new(),
            _ => self.x_data().to_vec(),
        }
    }

    /// Returns the grid data for grid-based series types.
    pub fn grid_data(&self) -> Option<&GridData> {
        match self {
            SeriesData::Heatmap { grid, .. }
            | SeriesData::Contour { grid, .. }
            | SeriesData::HeatmapContour { grid, .. } => Some(grid),
            _ => None,
        }
    }
}
