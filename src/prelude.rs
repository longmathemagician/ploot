pub use crate::api::{
    Figure, Layout2D, Layout3D, GridData, PlotOption, DashType, PointSymbol, SurfaceStyle, AxisPair,
    Plot2D, LinePlot, ScatterPlot, LinePointPlot, BarPlot, FillBetweenPlot, BoxAndWhiskerPlot, HeatmapPlot, ContourPlot, HeatmapContourPlot,
    HistogramPlot, Histogram2DPlot, CandlestickPlot, PiePlot
};
pub use crate::canvas::color::TermColor;
pub use crate::canvas::colormap::ColorMapType;
pub use crate::api::quick::{quick_contour, quick_heatmap, quick_plot, quick_plot_multi, quick_surface};
