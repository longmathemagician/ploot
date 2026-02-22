//! Public API types — [`Figure`], [`Axes2D`], and all option enums.

/// Axes configuration, annotations, and the [`Axes2D`] builder.
pub mod axes;
/// Top-level [`Figure`] type.
pub mod figure;
/// Option enums for plot, label, tick, and legend configuration.
pub mod options;
/// Series data storage ([`SeriesData`]).
pub mod series;

pub use axes::{Annotation, Arrow, Axes2D, AxisConfig, LegendConfig};
pub use figure::Figure;
pub use options::{
    AlignType, AutoOption, AxisPair, Coordinate, DashType, LabelOption, LegendOption, Placement,
    PlotOption, PointSymbol, TickOption,
};
pub use series::SeriesData;
