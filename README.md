# ploot

### Note: This is almost entirely machine generated code. It works for my applications but comes with absolutely no guarantees regarding API stability or anything else. Use at your own risk.

A terminal plotting library for Rust that renders charts using Unicode Braille characters (U+2800-U+28FF).

Each terminal cell encodes a 2x4 sub-pixel grid, giving smooth curves at high effective resolution without leaving the terminal.

![ploot rendering four mathematical functions with colored Braille characters](screenshot.png)

## Usage

Add `ploot` to your `Cargo.toml`:

```toml
[dependencies]
ploot = "0.1.2"
```

### Figure / Axes2D builder API

```rust
use ploot::{Figure, PlotOption, DashType, PointSymbol};

let mut fig = Figure::new();
fig.set_terminal_size(80, 24);
{
    let ax = fig.axes2d();
    ax.set_title("Builder API Demo");
    ax.set_x_label("x", &[]);
    ax.set_y_label("y", &[]);
    ax.set_y_grid(true);

    let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
    let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
    let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
    let gaussian: Vec<f64> = xs.iter().map(|&x| 20.0 * (-x * x).exp()).collect();

    ax.lines(
        xs.iter().copied(),
        quadratic.iter().copied(),
        &[PlotOption::Caption("x^2".into()), PlotOption::LineStyle(DashType::Solid)],
    );
    ax.lines(
        xs.iter().copied(),
        sine.iter().copied(),
        &[PlotOption::Caption("8*sin(1.5x)".into()), PlotOption::LineStyle(DashType::Dash)],
    );
    ax.points(
        xs.iter().copied(),
        gaussian.iter().copied(),
        &[PlotOption::Caption("20*e^(-x^2)".into()), PlotOption::PointSymbol(PointSymbol::Cross)],
    );
}
fig.show();
```

### Quick one-shot API

For simple plots without the full builder:

```rust
let xs: Vec<f64> = (0..=100).map(|i| i as f64 / 10.0).collect();
let ys: Vec<f64> = xs.iter().map(|&x| x.sin()).collect();

let plot = ploot::quick_plot(&xs, &ys, Some("sin(x)"), Some("x"), Some("y"), 80, 24);
println!("{plot}");
```

Plot multiple series with automatic color cycling:

```rust
let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
let cubic: Vec<f64> = xs.iter().map(|&x| x * x * x).collect();
let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
let gaussian: Vec<f64> = xs.iter().map(|&x| 20.0 * (-x * x).exp()).collect();

let plot = ploot::quick_plot_multi(
    &[(&xs, &quadratic), (&xs, &cubic), (&xs, &sine), (&xs, &gaussian)],
    Some("x^2  vs  x^3  vs  8*sin(1.5x)  vs  20*e^(-x^2)"),
    Some("x"),
    Some("y"),
    80,
    24,
);
println!("{plot}");
```

## Features

- **Braille rendering** - 2x4 sub-pixel resolution per terminal cell via bitwise dot compositing
- **Plot types** - lines, scatter (points), lines+points, bar charts (boxes), fill-between areas, error bars (X/Y), error lines (X/Y), box-and-whisker
- **Point symbols** - six marker styles: dot, cross, circle, diamond, triangle, square
- **ANSI color** - automatic 7-color palette cycling (blue, red, green, yellow, cyan, magenta, white), with additive color mixing when curves overlap
- **Axis layout** - auto-generated tick marks using Heckbert's nice numbers algorithm, dynamic label width computation, configurable title and axis labels
- **Secondary axes** - independent x2/y2 axes with separate ranges, labels, and tick marks
- **Logarithmic scaling** - per-axis log-scale support with any base
- **Grid lines** - major and minor grid with configurable color and dash patterns
- **Legend** - automatic legend with 4-corner placement control, optional title, horizontal/vertical layout
- **Line drawing** - Bresenham's algorithm with dash pattern support (solid, dash, dot, dot-dash, etc.)
- **Viewport clipping** - Cohen-Sutherland algorithm clips lines to the canvas bounds
- **Annotations** - text labels and arrows positioned in data or graph coordinates
- **Custom ticks** - user-defined tick positions and labels
- **Axis reversal** - flip axis direction
- **Multiplot** - subplot grid layout with shared super-title
- **LTTB downsampling** - automatic downsampling for large datasets
- **Terminal size detection** - auto-detect terminal dimensions via ioctl/env fallback
- **Zero dependencies** - pure Rust, no external crates

## Architecture

```
API (Figure, Axes2D, options, series data)
 └─ Render (series dispatching, rendering order, grid/legend/annotation overlay)
     └─ Layout (space allocation, tick generation, frame rendering)
         └─ Transform (data → normalized → pixel coordinate mapping, clipping, downsampling)
             └─ Canvas (BrailleCanvas, Bresenham lines, dash patterns, color compositing)
```

## License

Apache-2.0
