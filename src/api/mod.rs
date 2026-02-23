//! Public API types — [`Figure`], [`Axes2D`], and all option enums.

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
/// Series data storage ([`SeriesData`]).
pub mod series;

pub use axes::{Annotation, Arrow, Axes2D, AxisConfig, LegendConfig};
pub use axes3d::{Axes3D, SurfaceData, SurfaceStyle};
pub use figure::Figure;
pub use grid::GridData;
pub use options::{
    AlignType, AutoOption, AxisPair, Coordinate, DashType, LabelOption, LegendOption, Placement,
    PlotOption, PointSymbol, TickOption,
};
pub use series::SeriesData;
