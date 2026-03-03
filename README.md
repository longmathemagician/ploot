# ploot

### Note: This is almost entirely machine generated code. It works for my applications but comes with absolutely no guarantees regarding API stability or anything else. Use at your own risk.

A terminal plotting library for Rust that renders charts using Unicode Braille characters (U+2800-U+28FF).

Each terminal cell encodes a 2x4 sub-pixel grid, giving smooth curves at high effective resolution without leaving the terminal. Zero dependencies, pure Rust.

## Gallery

### Line plots with legend, grid, and dash styles

![Lines, points, and dash styles](screenshots/line_plot.svg)

### Multiple series with automatic color cycling

![Four mathematical functions plotted with auto color cycling](screenshots/multi_series.svg)

### Bar chart

![Bar chart with axis labels](screenshots/bar_chart.svg)

### Heatmap

Color-mapped density rendering of 2D scalar fields using Braille dot density + ANSI color.

![Heatmap of sin(x)*cos(y)](screenshots/heatmap.svg)

### Contour plot

Isolines extracted via marching squares, drawn as Braille curves.

![Contour plot of a Gaussian](screenshots/contour.svg)

### Heatmap + contour overlay

![Heatmap with contour lines overlaid](screenshots/heatmap_contour.svg)

### Dual Y-axis

Independent primary and secondary y-axes with automatic range detection.

![Dual Y-axis: temperature vs rainfall](screenshots/dual_axis.svg)

### 3D wireframe surface

![3D wireframe of sin(sqrt(x^2+y^2))](screenshots/surface_wireframe.svg)

### 3D hidden-line surface

Depth-buffered wireframe with per-pixel occlusion.

![3D hidden-line surface of sin(x)+cos(y)](screenshots/surface_hidden.svg)

### 3D filled surface

Color-mapped density shading with wireframe overlay.

![3D filled surface with rainbow colormap](screenshots/surface_filled.svg)

### Scatter plot

Six marker styles: dot, cross, circle, diamond, triangle, square.

![Scatter plot with multiple marker styles](screenshots/scatter.svg)

### Histogram

Automatic binning of raw data into bar counts.

![Histogram of simulated data](screenshots/histogram.svg)

### Box-and-whisker

Statistical summary plots showing median, quartiles, and range.

![Box-and-whisker plot](screenshots/box_whisker.svg)

### Confidence band (fill-between)

Shaded area between two curves for uncertainty visualization.

![Fill-between confidence band](screenshots/fill_between.svg)

### Candlestick chart

OHLC financial data rendered as candlestick bodies with high/low wicks.

![Candlestick chart](screenshots/candlestick.svg)

### Pie chart

Proportional arc segments with labels rendered via Braille density fill.

![Pie chart](screenshots/pie_chart.svg)

## Usage

Add `ploot` to your `Cargo.toml`:

```toml
[dependencies]
ploot = "0.1.5"
```

### Line plot

```rust
use ploot::prelude::*;

let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();

let layout = Layout2D::new()
    .with_title("Line Plot Demo")
    .with_x_label("x")
    .with_y_label("y")
    .with_y_grid(true)
    .with_plot(LinePlot::new(xs.iter().copied(), quadratic.iter().copied())
        .with_caption("x^2"))
    .with_plot(LinePlot::new(xs.iter().copied(), sine.iter().copied())
        .with_caption("8*sin(1.5x)")
        .with_dash(DashType::Dash));

Figure::new()
    .with_size(80, 24)
    .with_layout(layout)
    .show();
```

### Scatter plot

```rust
use ploot::prelude::*;

let xs: Vec<f64> = (0..40).map(|i| (i as f64 * 0.15).sin() * 3.0).collect();
let ys: Vec<f64> = (0..40).map(|i| (i as f64 * 0.15).cos() * 3.0).collect();

let layout = Layout2D::new()
    .with_title("Scatter")
    .with_plot(ScatterPlot::new(xs, ys)
        .with_symbol(PointSymbol::Circle)
        .with_caption("points"));

Figure::new()
    .with_size(60, 20)
    .with_layout(layout)
    .show();
```

### Heatmap

```rust
use ploot::prelude::*;

let grid = GridData::from_fn(
    |x, y| x.sin() * y.cos(),
    (-std::f64::consts::PI, std::f64::consts::PI),
    (-std::f64::consts::PI, std::f64::consts::PI),
    40, 40,
);

let layout = Layout2D::new()
    .with_title("sin(x)*cos(y)")
    .with_x_label("x")
    .with_y_label("y")
    .with_plot(HeatmapPlot::new(grid));

Figure::new()
    .with_size(80, 24)
    .with_layout(layout)
    .show();
```

### Contour

```rust
use ploot::prelude::*;

let grid = GridData::from_fn(
    |x, y| (-0.5 * (x * x + y * y)).exp(),
    (-3.0, 3.0), (-3.0, 3.0), 40, 40,
);

let layout = Layout2D::new()
    .with_title("Gaussian")
    .with_plot(ContourPlot::new(grid).with_levels(8));

Figure::new()
    .with_size(80, 24)
    .with_layout(layout)
    .show();
```

### Heatmap + contour overlay

```rust
use ploot::prelude::*;

let grid = GridData::from_fn(
    |x, y| x.sin() * y.cos(),
    (-std::f64::consts::PI, std::f64::consts::PI),
    (-std::f64::consts::PI, std::f64::consts::PI),
    40, 40,
);

let layout = Layout2D::new()
    .with_title("Heatmap + Contour")
    .with_plot(HeatmapContourPlot::new(grid)
        .with_levels(10)
        .with_colormap(ColorMapType::BlueRed));

Figure::new()
    .with_size(80, 24)
    .with_layout(layout)
    .show();
```

### 3D surface

```rust
use ploot::prelude::*;

let grid = GridData::from_fn(
    |x, y| (x * x + y * y).sqrt().sin(),
    (-4.0, 4.0), (-4.0, 4.0), 20, 20,
);

let layout = Layout3D::new()
    .with_title("3D Surface")
    .with_view(30.0, 30.0)
    .with_colormap(ColorMapType::Rainbow)
    .with_surface(grid, SurfaceStyle::Filled, &[]);

Figure::new()
    .with_size(80, 24)
    .with_layout3d(layout)
    .show();
```

Other surface styles: `SurfaceStyle::Wireframe`, `SurfaceStyle::HiddenLine`.

### Dual Y-axis

```rust
use ploot::prelude::*;

let months: Vec<f64> = (1..=12).map(|i| i as f64).collect();
let temperature = vec![-2.0, 0.5, 5.0, 11.0, 16.0, 20.0, 22.0, 21.0, 17.0, 11.0, 5.0, 0.0];
let rainfall = vec![45.0, 40.0, 55.0, 60.0, 70.0, 80.0, 75.0, 65.0, 70.0, 75.0, 60.0, 50.0];

let layout = Layout2D::new()
    .with_title("Monthly Temperature & Rainfall")
    .with_x_label("Month")
    .with_y_label("Temperature (C)")
    .with_plot(LinePlot::new(months.iter().copied(), temperature.iter().copied())
        .with_caption("Temperature")
        .with_color(TermColor::Red))
    .with_plot(LinePlot::new(months.iter().copied(), rainfall.iter().copied())
        .with_caption("Rainfall")
        .with_color(TermColor::Blue)
        .with_axes(AxisPair::X1Y2)
        .with_dash(DashType::Dash));

Figure::new()
    .with_size(80, 24)
    .with_layout(layout)
    .show();
```

### Quick one-shot API

For throwaway plots without the builder:

```rust
let xs: Vec<f64> = (0..=100).map(|i| i as f64 / 10.0).collect();
let ys: Vec<f64> = xs.iter().map(|&x| x.sin()).collect();

let output = ploot::quick_plot(&xs, &ys, Some("sin(x)"), Some("x"), Some("y"), 80, 24);
println!("{output}");
```

## Features

### 2D plots
- **Line plots** - solid, dashed, dotted, and custom dash patterns
- **Scatter plots** - six marker styles: dot, cross, circle, diamond, triangle, square
- **Bar charts** - filled boxes with configurable width
- **Histograms** - automatic binning of raw data with optional normalization
- **Fill-between** - shaded area between two curves
- **Error bars** - X and Y error bars, with or without connecting lines
- **Box-and-whisker** - statistical summary plots
- **Candlestick** - OHLC financial data with body and wick rendering
- **Pie chart** - proportional arc segments with labels

### 2D grid data
- **Heatmap** - color-mapped density rendering of Z(x,y) scalar fields using Braille dot density + ANSI color (7 colors x 8 density levels = 56 perceptual levels)
- **Contour** - isolines via marching squares with linear interpolation and saddle-point disambiguation
- **Heatmap + contour overlay** - combined density shading with isoline overlay

### 3D surfaces
- **Wireframe** - projected mesh lines from configurable azimuth/elevation viewpoint
- **Hidden-line** - depth-buffered wireframe with per-pixel z-test occlusion
- **Filled** - color-mapped density shading per quad with wireframe overlay
- **Colormaps** - Heat, Gray, Rainbow, BlueRed

### Rendering
- **Braille rendering** - 2x4 sub-pixel resolution per terminal cell via bitwise dot compositing
- **ANSI color** - automatic 7-color palette cycling (blue, red, green, yellow, cyan, magenta, white), with additive color mixing when curves overlap
- **Line drawing** - Bresenham's algorithm with dash pattern support
- **Viewport clipping** - Cohen-Sutherland algorithm clips lines to canvas bounds
- **Depth buffer** - z-buffer for correct 3D occlusion (front-to-back rendering)

### Layout and axes
- **Axis layout** - auto-generated tick marks using Heckbert's nice numbers algorithm
- **Secondary axes** - independent x2/y2 axes with separate ranges, labels, and tick marks
- **Logarithmic scaling** - per-axis log-scale support with any base
- **Grid lines** - major and minor grid with configurable color and dash patterns
- **Legend** - automatic legend with 4-corner placement control
- **Annotations** - text labels and arrows in data or graph coordinates
- **Custom ticks** - user-defined tick positions and labels
- **Axis reversal** - flip axis direction
- **Multiplot** - subplot grid layout with shared super-title (2D and 3D can be mixed)

### Data handling
- **LTTB downsampling** - automatic downsampling for large datasets
- **GridData** - 2D matrix with `from_fn` (sample a function) and `from_rows` (nested iterators) constructors, bilinear interpolation
- **Terminal size detection** - auto-detect terminal dimensions via ioctl/env fallback
- **Zero dependencies** - pure Rust, no external crates

## Architecture

```
API (Figure, Axes2D, Axes3D, GridData, options, series data)
 |
 +-- Render
 |    +-- 2D: lines, points, boxes, fill, error_bars, box_whisker, heatmap, contour
 |    +-- 3D: surface (wireframe, hidden-line, filled)
 |    +-- Overlays: grid, legend, annotations
 |
 +-- Layout (space allocation, tick generation, frame rendering)
 |
 +-- Transform
 |    +-- CoordinateMapper (data -> pixel), clipping, downsampling
 |    +-- Projection (3D -> 2D rotation), marching squares
 |
 +-- Canvas
      +-- BrailleCanvas (Bresenham lines, dash patterns, color compositing)
      +-- DepthCanvas (z-buffer wrapper for 3D occlusion)
      +-- ColorMap (value -> color + density mapping)
```

## Examples

Run any example from the `examples/` directory:

```bash
cargo run --example bar_chart
cargo run --example box_whisker
cargo run --example candlestick
cargo run --example contour
cargo run --example dual_axis
cargo run --example fill_between
cargo run --example heatmap
cargo run --example heatmap_contour
cargo run --example histogram
cargo run --example line_plot
cargo run --example multi_series
cargo run --example pie_chart
cargo run --example scatter
cargo run --example surface_filled
cargo run --example surface_hidden
cargo run --example surface_wireframe
```

## License

Apache-2.0
