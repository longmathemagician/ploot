//! Public API types.
//!
//! The recommended way to build plots is with typed builder structs
//! ([`LinePlot`], [`ScatterPlot`], [`HeatmapPlot`], …) composed into a
//! [`Layout2D`] or [`Layout3D`], then rendered via [`Figure`].
//!
//! A gnuplot-compatible mutable-borrow API ([`Figure::axes2d`] → [`Axes2D`])
//! is also available for callers that prefer that style.

/// Axes configuration, annotations, and the [`Axes2D`] builder.
pub mod axes;
/// 3D axes builder.
pub mod axes3d;
/// Top-level [`Figure`] type.
pub mod figure;
/// 2D grid data for heatmaps, contours, and surfaces.
pub mod grid;
/// Option enums for plot, label, tick, and legend configuration.
pub mod options;
/// Plot builder structs (kuva-style).
pub mod plots;
/// Quick one-shot convenience functions.
pub mod quick;
/// Series data storage ([`SeriesData`]).
pub mod series;

pub use axes::{Annotation, Arrow, Axes2D, AxisConfig, Layout2D, LegendConfig};
pub use quick::{quick_contour, quick_heatmap, quick_plot, quick_plot_multi, quick_surface};
pub use axes3d::{Axes3D, Layout3D, SurfaceData, SurfaceStyle};
pub use figure::Figure;
pub use grid::GridData;
pub use options::{
    AlignType, AutoOption, AxisPair, Coordinate, DashType, LabelOption, LegendOption, Placement,
    PlotOption, PointSymbol, TickOption,
};
pub use series::SeriesData;
pub use plots::{
    BarPlot, BoxAndWhiskerPlot, ContourPlot, FillBetweenPlot, HeatmapContourPlot, HeatmapPlot,
    LinePlot, LinePointPlot, Plot2D, ScatterPlot, HistogramPlot, Histogram2DPlot, CandlestickPlot, PiePlot,
};
