use crate::canvas::DashPattern;
use crate::canvas::color::TermColor;
use crate::canvas::colormap::ColorMapType;

/// Auto-or-fixed value for axis configuration.
///
/// # Examples
///
/// ```
/// use ploot::AutoOption;
///
/// let auto: AutoOption<f64> = AutoOption::Auto;
/// let fixed: AutoOption<f64> = AutoOption::Fix(42.0);
/// ```
#[derive(Debug, Clone)]
pub enum AutoOption<T> {
    /// Automatically determined.
    Auto,
    /// Fixed to a specific value.
    Fix(T),
}

/// Which axis pair a series is plotted against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AxisPair {
    /// Primary x and y axes (bottom, left).
    #[default]
    X1Y1,
    /// Primary x, secondary y (bottom, right).
    X1Y2,
    /// Secondary x, primary y (top, left).
    X2Y1,
    /// Secondary x and y axes (top, right).
    X2Y2,
}

/// Dash type for line rendering.
///
/// # Examples
///
/// ```
/// use ploot::{PlotOption, DashType};
///
/// let opts = [PlotOption::LineStyle(DashType::Dash)];
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashType {
    /// Continuous line (every pixel drawn).
    Solid,
    /// Long dashes (6 on, 4 off).
    Dash,
    /// Dots (2 on, 4 off).
    Dot,
    /// Alternating dot-dash (2-4-6-4).
    DotDash,
    /// Dot-dot-dash pattern (2-4-2-4-6-4).
    DotDotDash,
    /// Fine dots (1 on, 3 off) — used for grid lines.
    SmallDot,
}

impl DashType {
    /// Converts to the canvas layer's DashPattern.
    pub fn to_pattern(self) -> &'static DashPattern {
        use crate::canvas;
        match self {
            DashType::Solid => &canvas::SOLID,
            DashType::Dash => &canvas::DASH,
            DashType::Dot => &canvas::DOT,
            DashType::DotDash => &canvas::DOT_DASH,
            DashType::DotDotDash => &canvas::DOT_DOT_DASH,
            DashType::SmallDot => &canvas::SMALL_DOT,
        }
    }
}

/// Point marker symbol for scatter plots.
///
/// # Examples
///
/// ```
/// use ploot::{PlotOption, PointSymbol};
///
/// let opts = [PlotOption::PointSymbol(PointSymbol::Diamond)];
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PointSymbol {
    /// Single Braille dot.
    #[default]
    Dot,
    /// Small cross pattern (5 dots).
    Cross,
    /// Small circle pattern (ring of dots).
    Circle,
    /// Small diamond pattern.
    Diamond,
    /// Small triangle pattern.
    Triangle,
    /// Small square pattern.
    Square,
}

/// Per-series plot options.
///
/// # Examples
///
/// ```
/// use ploot::{PlotOption, DashType};
/// use ploot::canvas::color::TermColor;
///
/// let opts = [
///     PlotOption::Caption("temperature".into()),
///     PlotOption::Color(TermColor::Red),
///     PlotOption::LineStyle(DashType::Dash),
/// ];
/// ```
#[derive(Debug, Clone)]
pub enum PlotOption {
    /// Series label shown in legend.
    Caption(String),
    /// Series color (overrides auto palette).
    Color(TermColor),
    /// Dash pattern for lines.
    LineStyle(DashType),
    /// Line width (1 = normal, 2+ = denser dash patterns).
    LineWidth(u32),
    /// Point marker symbol.
    PointSymbol(PointSymbol),
    /// Which axis pair to plot against.
    Axes(AxisPair),
    /// Border color for boxes.
    BorderColor(TermColor),
    /// Box width as fraction of spacing (0.0-1.0).
    BoxWidth(f64),
    /// Colormap for heatmap/surface rendering.
    ColorMap(ColorMapType),
    /// Number of auto-generated contour levels.
    ContourLevels(usize),
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignType {
    /// Left-aligned text.
    AlignLeft,
    /// Right-aligned text.
    AlignRight,
    /// Center-aligned text.
    AlignCenter,
    /// Top-aligned text.
    AlignTop,
    /// Bottom-aligned text.
    AlignBottom,
}

/// Options for axis labels and annotations.
#[derive(Debug, Clone)]
pub enum LabelOption {
    /// Offset from default position (dx, dy) in character cells.
    TextOffset(f64, f64),
    /// Text color.
    TextColor(TermColor),
    /// Rotation angle in degrees.
    Rotate(f64),
    /// Text alignment.
    TextAlign(AlignType),
    /// Marker symbol for label position.
    MarkerSymbol(PointSymbol),
    /// Marker color.
    MarkerColor(TermColor),
}

/// Options for axis ticks.
#[derive(Debug, Clone)]
pub enum TickOption {
    /// Place ticks on the axis line rather than the border.
    OnAxis,
    /// Mirror ticks to the opposite border.
    Mirror,
    /// Draw ticks inward (default is outward).
    Inward,
    /// Printf-style format string for tick labels.
    Format(String),
}

/// Coordinate space for positioning.
///
/// # Examples
///
/// ```
/// use ploot::Coordinate;
///
/// // Center of the plot area
/// let center = Coordinate::Graph(0.5);
/// // At x=42 in data coordinates
/// let data_pos = Coordinate::Axis(42.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Coordinate {
    /// Normalized graph coordinates (0.0 = left/bottom, 1.0 = right/top).
    Graph(f64),
    /// Primary axis data coordinates.
    Axis(f64),
    /// Secondary axis data coordinates.
    Axis2(f64),
}

/// Legend placement position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    /// Top-right corner (default).
    #[default]
    TopRight,
    /// Top-left corner.
    TopLeft,
    /// Bottom-right corner.
    BottomRight,
    /// Bottom-left corner.
    BottomLeft,
}

/// Options for legend display.
#[derive(Debug, Clone)]
pub enum LegendOption {
    /// Reverse the order of entries.
    Reverse,
    /// Horizontal layout instead of vertical.
    Horizontal,
    /// Legend placement corner.
    Placement(Placement),
    /// Legend title.
    Title(String),
}
