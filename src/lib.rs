//! Terminal plotting with Unicode Braille characters.
//!
//! **ploot** renders charts in the terminal using Unicode Braille characters
//! (U+2800–U+28FF), giving 2×4 sub-pixel resolution per terminal cell.
//! Pure Rust, no external processes.
//!
//! Build plots with typed builder structs ([`LinePlot`](api::plots::LinePlot),
//! [`ScatterPlot`](api::plots::ScatterPlot), [`BarPlot`](api::plots::BarPlot), …)
//! composed into a [`Layout2D`](api::axes::Layout2D) or [`Layout3D`](api::axes3d::Layout3D),
//! then rendered via [`Figure`].
//!
//! A gnuplot-compatible mutable-borrow API ([`Figure::axes2d`], [`Axes2D::lines`], …)
//! is also available.
//!
//! # Features
//!
//! - Line, scatter, bar, histogram, candlestick, and pie chart plot types
//! - Fill-between, error bars, and box-and-whisker plots
//! - Heatmaps, contour plots, and heatmap+contour overlays
//! - 3D surfaces: wireframe, hidden-line, and color-mapped filled
//! - Grid and minor grid with configurable dash patterns
//! - Legend with placement control
//! - Logarithmic axis scaling
//! - Secondary (x2/y2) axes
//! - Multiplot (subplot grid) layout
//! - LTTB auto-downsampling for large datasets
//! - Annotations and arrows
//! - Custom tick positions and labels
//! - SVG export
//!
//! # Quick start
//!
//! ```
//! use ploot::prelude::*;
//!
//! let xs: Vec<f64> = (0..=60).map(|i| i as f64 / 10.0).collect();
//! let ys: Vec<f64> = xs.iter().map(|x| x.sin()).collect();
//!
//! let layout = Layout2D::new()
//!     .with_title("Sine wave")
//!     .with_x_label("x")
//!     .with_y_label("y")
//!     .with_plot(LinePlot::new(xs, ys).with_caption("sin(x)"));
//!
//! let output = Figure::new()
//!     .with_size(60, 15)
//!     .with_layout(layout)
//!     .render();
//! assert!(!output.is_empty());
//! ```
//!
//! For one-off plots without the full builder API, use [`quick_plot`]:
//!
//! ```
//! let xs: Vec<f64> = (0..=20).map(|i| i as f64).collect();
//! let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
//! let output = ploot::quick_plot(&xs, &ys, Some("x²"), Some("x"), Some("y"), 60, 15);
//! assert!(output.contains("x²"));
//! ```

#![warn(missing_docs)]

pub mod api;
pub mod canvas;
pub mod layout;
pub mod render;
/// Terminal size detection.
pub mod terminal;
pub mod transform;
pub mod export;
/// Convenient glob import of the most commonly needed types.
pub mod prelude;

// Re-export the public API types
pub use api::{
    AlignType, AutoOption, Axes2D, Axes3D, AxisPair, Coordinate, DashType, Figure, GridData,
    LabelOption, LegendOption, Placement, PlotOption, PointSymbol, SeriesData, SurfaceStyle,
    TickOption, quick_contour, quick_heatmap, quick_plot, quick_plot_multi, quick_surface,
};
pub use canvas::colormap::ColorMapType;

#[cfg(test)]
mod tests {
    use super::*;

    // Figure/Axes API tests

    #[test]
    fn figure_api_basic() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_title("Figure API");
            ax.set_x_label("x", &[]);
            ax.set_y_label("y", &[]);
            let xs: Vec<f64> = (0..=20).map(|i| i as f64 / 2.0).collect();
            let ys: Vec<f64> = xs.iter().map(|x| x.sin()).collect();
            ax.lines(
                xs.iter().copied(),
                ys.iter().copied(),
                &[
                    PlotOption::Caption("sin(x)".into()),
                    PlotOption::Color(canvas::color::TermColor::Blue),
                ],
            );
        }
        let result = fig.render();
        assert!(result.contains("Figure API"));
        assert!(!result.is_empty());
    }

    #[test]
    fn figure_with_grid() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_x_grid(true);
            ax.set_y_grid(true);
            let xs = vec![0.0, 5.0, 10.0];
            let ys = vec![0.0, 10.0, 5.0];
            ax.lines(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        let result = fig.render();
        assert!(!result.is_empty());
    }

    #[test]
    fn figure_with_legend() {
        let mut fig = Figure::new();
        fig.set_terminal_size(80, 20);
        {
            let ax = fig.axes2d();
            let xs = vec![0.0, 1.0, 2.0, 3.0];
            let ys1 = vec![0.0, 1.0, 4.0, 9.0];
            let ys2 = vec![0.0, 2.0, 4.0, 6.0];
            ax.lines(
                xs.iter().copied(),
                ys1.iter().copied(),
                &[PlotOption::Caption("quadratic".into())],
            );
            ax.lines(
                xs.iter().copied(),
                ys2.iter().copied(),
                &[PlotOption::Caption("linear".into())],
            );
        }
        let result = fig.render();
        assert!(result.contains("quadratic"));
        assert!(result.contains("linear"));
    }

    // Grid / heatmap / contour / surface integration tests

    #[test]
    fn heatmap_via_figure() {
        let grid = GridData::from_fn(|x, y| x.sin() * y.cos(), (-3.0, 3.0), (-3.0, 3.0), 20, 20);
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_title("Heatmap");
            ax.heatmap(grid, &[]);
        }
        let result = fig.render();
        assert!(result.contains("Heatmap"));
        assert!(!result.is_empty());
    }

    #[test]
    fn contour_via_figure() {
        let grid = GridData::from_fn(|x, y| x * x + y * y, (-2.0, 2.0), (-2.0, 2.0), 20, 20);
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_title("Contour");
            ax.contour(grid, None, &[PlotOption::ContourLevels(5)]);
        }
        let result = fig.render();
        assert!(result.contains("Contour"));
    }

    #[test]
    fn heatmap_contour_via_figure() {
        let grid = GridData::from_fn(|x, y| x.sin() * y.cos(), (-3.0, 3.0), (-3.0, 3.0), 20, 20);
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_title("HeatContour");
            ax.heatmap_contour(grid, None, &[]);
        }
        let result = fig.render();
        assert!(result.contains("HeatContour"));
    }

    #[test]
    fn surface_wireframe_via_figure() {
        let grid = GridData::from_fn(
            |x, y| (x * x + y * y).sqrt().sin(),
            (-3.0, 3.0),
            (-3.0, 3.0),
            20,
            20,
        );
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 20);
        {
            let ax = fig.axes3d();
            ax.set_title("Wireframe");
            ax.set_view(30.0, 30.0);
            ax.surface(grid, SurfaceStyle::Wireframe, &[]);
        }
        let result = fig.render();
        assert!(result.contains("Wireframe"));
    }

    #[test]
    fn surface_hidden_via_figure() {
        let grid = GridData::from_fn(|x, y| x.sin() + y.cos(), (-3.0, 3.0), (-3.0, 3.0), 15, 15);
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 20);
        {
            let ax = fig.axes3d();
            ax.set_title("Hidden");
            ax.surface(grid, SurfaceStyle::HiddenLine, &[]);
        }
        let result = fig.render();
        assert!(result.contains("Hidden"));
    }

    #[test]
    fn surface_filled_via_figure() {
        let grid = GridData::from_fn(|x, y| x.sin() * y.cos(), (-3.0, 3.0), (-3.0, 3.0), 15, 15);
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 20);
        {
            let ax = fig.axes3d();
            ax.set_title("Filled");
            ax.set_colormap(ColorMapType::Rainbow);
            ax.surface(grid, SurfaceStyle::Filled, &[]);
        }
        let result = fig.render();
        assert!(result.contains("Filled"));
    }

    #[test]
    fn mixed_multiplot_2d_and_3d() {
        let mut fig = Figure::new();
        fig.set_terminal_size(80, 24);
        fig.set_multiplot_layout(1, 2);
        {
            let ax = fig.axes2d();
            ax.set_title("2D");
            let xs = vec![0.0, 1.0, 2.0];
            let ys = vec![0.0, 1.0, 4.0];
            ax.lines(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        {
            let ax = fig.axes3d();
            ax.set_title("3D");
            let grid = GridData::from_fn(|x, y| x + y, (0.0, 1.0), (0.0, 1.0), 5, 5);
            ax.surface(grid, SurfaceStyle::Wireframe, &[]);
        }
        let result = fig.render();
        assert!(result.contains("2D"));
        assert!(result.contains("3D"));
    }
}
