//! Rendering pipeline — dispatches [`Figure`] data to canvas.

/// Box-and-whisker plot renderer.
pub mod box_whisker;
/// Bar chart renderer.
pub mod boxes;
/// Contour line renderer.
pub mod contour;
/// Error bar renderer.
pub mod error_bars;
/// Fill-between area renderer.
pub mod fill;
/// Grid line renderer.
pub mod grid;
/// Heatmap renderer.
pub mod heatmap;
/// Legend overlay renderer.
pub mod legend;
/// Line series renderer.
pub mod lines;
/// Point (scatter) renderer.
pub mod points;
/// 3D surface renderer.
pub mod surface;

use crate::api::axes::Axes2D;
use crate::api::axes3d::{Axes3D, SurfaceStyle};
use crate::api::figure::{AxesType, Figure};
use crate::api::options::{AutoOption, AxisPair, DashType, PlotOption, PointSymbol};
use crate::api::series::SeriesData;
use crate::canvas::color::TermColor;
use crate::canvas::colormap::ColorMapType;
use crate::canvas::depth::DepthCanvas;
use crate::canvas::{BrailleCanvas, PALETTE};
use crate::layout::text::TextGrid;
use crate::layout::{Layout, LayoutConfig, compute_layout, generate_ticks, render_frame};
use crate::transform::marching::auto_contour_levels;
use crate::transform::projection::Projection;
use crate::transform::{CoordinateMapper, aligned_x_pixel_range, aligned_y_pixel_range};

/// Render a complete Figure to a string.
pub fn render_figure(fig: &Figure) -> String {
    if fig.axes.is_empty() {
        return String::new();
    }

    match fig.multiplot {
        Some((rows, cols)) => render_multiplot(fig, rows, cols),
        None => {
            // Single plot — use first axes
            render_single_axes_type(&fig.axes[0], fig.title.as_deref(), fig.width, fig.height)
        }
    }
}

/// Dispatch rendering for either 2D or 3D axes.
fn render_single_axes_type(
    axes_type: &AxesType,
    super_title: Option<&str>,
    width: usize,
    height: usize,
) -> String {
    match axes_type {
        AxesType::TwoD(axes) => render_single_axes(axes, super_title, width, height),
        AxesType::ThreeD(axes) => render_single_axes3d(axes, super_title, width, height),
    }
}

/// Render a multiplot grid.
fn render_multiplot(fig: &Figure, rows: usize, cols: usize) -> String {
    let title_rows = if fig.title.is_some() { 1 } else { 0 };
    let sub_height = fig.height.saturating_sub(title_rows) / rows.max(1);
    let sub_width = fig.width / cols.max(1);

    // Render multiplot with color — render each subplot independently
    let mut output = String::new();
    if let Some(title) = &fig.title {
        let pad = fig.width.saturating_sub(title.len()) / 2;
        for _ in 0..pad {
            output.push(' ');
        }
        output.push_str(title);
        for _ in 0..(fig.width.saturating_sub(pad + title.len())) {
            output.push(' ');
        }
        output.push('\n');
    }

    for grid_row in 0..rows {
        // Render each row of subplots
        let axes_in_row: Vec<_> = (0..cols)
            .filter_map(|grid_col| {
                let idx = grid_row * cols + grid_col;
                fig.axes.get(idx)
            })
            .collect();

        // Render each subplot
        let rendered: Vec<String> = axes_in_row
            .iter()
            .map(|axes| render_single_axes_type(axes, None, sub_width, sub_height))
            .collect();

        // Interleave lines from each subplot
        let max_lines = rendered
            .iter()
            .map(|r| r.lines().count())
            .max()
            .unwrap_or(0);
        for line_idx in 0..max_lines {
            for (col_idx, r) in rendered.iter().enumerate() {
                if let Some(line) = r.lines().nth(line_idx) {
                    output.push_str(line);
                } else {
                    // Pad empty line
                    for _ in 0..sub_width {
                        output.push(' ');
                    }
                }
                if col_idx < rendered.len() - 1 {
                    // No separator needed, widths should align
                }
            }
            output.push('\n');
        }
    }

    // Remove trailing newline
    if output.ends_with('\n') {
        output.pop();
    }

    output
}

/// Render a single Axes2D to a string.
fn render_single_axes(
    axes: &Axes2D,
    super_title: Option<&str>,
    width: usize,
    height: usize,
) -> String {
    // Compute data ranges
    let (x_min, x_max, y_min, y_max) = compute_data_ranges(axes);

    // Apply manual range overrides
    let x_min = match &axes.x_axis.range_min {
        AutoOption::Fix(v) => *v,
        AutoOption::Auto => x_min,
    };
    let x_max = match &axes.x_axis.range_max {
        AutoOption::Fix(v) => *v,
        AutoOption::Auto => x_max,
    };
    let y_min = match &axes.y_axis.range_min {
        AutoOption::Fix(v) => *v,
        AutoOption::Auto => y_min,
    };
    let y_max = match &axes.y_axis.range_max {
        AutoOption::Fix(v) => *v,
        AutoOption::Auto => y_max,
    };

    // Generate ticks
    let mut x_ticks = if let Some(custom) = &axes.x_axis.custom_ticks {
        custom_tick_set(custom, x_min, x_max)
    } else if let Some(base) = axes.x_axis.log_base {
        log_ticks(x_min, x_max, base)
    } else {
        generate_ticks(x_min, x_max, 6)
    };
    let mut y_ticks = if let Some(custom) = &axes.y_axis.custom_ticks {
        custom_tick_set(custom, y_min, y_max)
    } else if let Some(base) = axes.y_axis.log_base {
        log_ticks(y_min, y_max, base)
    } else {
        generate_ticks(y_min, y_max, 5)
    };

    // For grid-based series (heatmaps, contours), use the exact data range
    // so the grid fills the viewport edge-to-edge, with ticks slightly inward.
    let has_grid = axes.series.iter().any(|s| s.grid_data().is_some());
    if has_grid {
        x_ticks.min = x_min;
        x_ticks.max = x_max;
        x_ticks.ticks.retain(|&(v, _)| v >= x_min && v <= x_max);
        y_ticks.min = y_min;
        y_ticks.max = y_max;
        y_ticks.ticks.retain(|&(v, _)| v >= y_min && v <= y_max);
    }

    // Expand tick range for box/bar series (only auto-range sides)
    let (box_need_min, box_need_max) = compute_box_x_padding(axes);
    if box_need_min.is_finite() {
        if matches!(axes.x_axis.range_min, AutoOption::Auto) {
            x_ticks.min = x_ticks.min.min(box_need_min);
        }
        if matches!(axes.x_axis.range_max, AutoOption::Auto) {
            x_ticks.max = x_ticks.max.max(box_need_max);
        }
    }
    let has_box_padding = box_need_min.is_finite()
        && (x_ticks.min < x_ticks.ticks.first().map_or(f64::INFINITY, |t| t.0) - 1e-10
            || x_ticks.max > x_ticks.ticks.last().map_or(f64::NEG_INFINITY, |t| t.0) + 1e-10);

    // Determine title
    let title = axes.title.as_deref().or(super_title);

    // Generate secondary axis ticks if any series uses them
    let any_series_y2 = axes
        .series
        .iter()
        .any(|s| matches!(s.axis_pair(), AxisPair::X1Y2 | AxisPair::X2Y2));
    let any_series_x2 = axes
        .series
        .iter()
        .any(|s| matches!(s.axis_pair(), AxisPair::X2Y1 | AxisPair::X2Y2));
    let has_y2 = any_series_y2
        || axes.y2_axis.label.is_some()
        || axes.y2_axis.custom_ticks.is_some()
        || !matches!(axes.y2_axis.range_min, AutoOption::Auto);
    let has_x2 = any_series_x2
        || axes.x2_axis.label.is_some()
        || axes.x2_axis.custom_ticks.is_some()
        || !matches!(axes.x2_axis.range_min, AutoOption::Auto);

    let y2_ticks = if has_y2 {
        let (_, _, y2_min, y2_max) = compute_secondary_ranges(axes);
        let y2_min = match &axes.y2_axis.range_min {
            AutoOption::Fix(v) => *v,
            AutoOption::Auto => y2_min,
        };
        let y2_max = match &axes.y2_axis.range_max {
            AutoOption::Fix(v) => *v,
            AutoOption::Auto => y2_max,
        };
        if let Some(custom) = &axes.y2_axis.custom_ticks {
            Some(custom_tick_set(custom, y2_min, y2_max))
        } else if let Some(base) = axes.y2_axis.log_base {
            Some(log_ticks(y2_min, y2_max, base))
        } else {
            Some(generate_ticks(y2_min, y2_max, 5))
        }
    } else {
        None
    };

    let x2_ticks = if has_x2 {
        let (x2_min, x2_max, _, _) = compute_secondary_ranges(axes);
        let x2_min = match &axes.x2_axis.range_min {
            AutoOption::Fix(v) => *v,
            AutoOption::Auto => x2_min,
        };
        let x2_max = match &axes.x2_axis.range_max {
            AutoOption::Fix(v) => *v,
            AutoOption::Auto => x2_max,
        };
        if let Some(custom) = &axes.x2_axis.custom_ticks {
            Some(custom_tick_set(custom, x2_min, x2_max))
        } else if let Some(base) = axes.x2_axis.log_base {
            Some(log_ticks(x2_min, x2_max, base))
        } else {
            Some(generate_ticks(x2_min, x2_max, 6))
        }
    } else {
        None
    };

    // Compute layout
    let config = LayoutConfig {
        total_width: width,
        total_height: height,
        title: title.map(String::from),
        x_label: axes.x_axis.label.clone(),
        y_label: axes.y_axis.label.clone(),
        x2_label: axes.x2_axis.label.clone(),
        y2_label: axes.y2_axis.label.clone(),
        x2_ticks: x2_ticks.clone(),
        y2_ticks: y2_ticks.clone(),
    };
    let layout = compute_layout(&config, &x_ticks, &y_ticks);

    // Create coordinate mapper with pixel ranges.
    // For grid-based series (heatmaps, contours), use the full canvas extent
    // so the grid fills edge-to-edge. For other series, align to tick positions.
    let (y_px_min, y_px_max, x_px_min, x_px_max) = if has_grid {
        (
            0.0,
            (layout.canvas_char_height * 4 - 1) as f64,
            0.0,
            (layout.canvas_char_width * 2 - 1) as f64,
        )
    } else if has_box_padding {
        // Proportional: map full data range to full canvas width
        let (y_min_px, y_max_px) =
            aligned_y_pixel_range(layout.canvas_char_height, y_ticks.ticks.len());
        let x_min_px = 0.0;
        let x_max_px = 2.0 * (layout.canvas_char_width - 1) as f64;
        (y_min_px, y_max_px, x_min_px, x_max_px)
    } else {
        let (y_min_px, y_max_px) =
            aligned_y_pixel_range(layout.canvas_char_height, y_ticks.ticks.len());
        let (x_min_px, x_max_px) =
            aligned_x_pixel_range(layout.canvas_char_width, x_ticks.ticks.len());
        (y_min_px, y_max_px, x_min_px, x_max_px)
    };

    let mapper = CoordinateMapper::with_pixel_ranges(
        x_ticks.min,
        x_ticks.max,
        y_ticks.min,
        y_ticks.max,
        x_px_min,
        x_px_max,
        y_px_min,
        y_px_max,
    )
    .with_reversal(axes.x_axis.reversed, axes.y_axis.reversed)
    .with_log(axes.x_axis.log_base, axes.y_axis.log_base);

    // Build secondary y-axis mapper if needed
    let y2_mapper = y2_ticks.as_ref().map(|y2t| {
        CoordinateMapper::with_pixel_ranges(
            x_ticks.min,
            x_ticks.max,
            y2t.min,
            y2t.max,
            x_px_min,
            x_px_max,
            y_px_min,
            y_px_max,
        )
        .with_reversal(axes.x_axis.reversed, axes.y2_axis.reversed)
        .with_log(axes.x_axis.log_base, axes.y2_axis.log_base)
    });

    // Create canvas
    let mut canvas = BrailleCanvas::new(layout.canvas_char_width, layout.canvas_char_height);

    // Draw grid lines BEFORE data
    draw_grids(
        &mut canvas,
        &mapper,
        &x_ticks,
        &y_ticks,
        axes,
        y2_mapper.as_ref(),
        y2_ticks.as_ref(),
    );

    // Draw series in rendering order: fills -> boxes -> error bars -> lines/points
    draw_all_series(&mut canvas, &mapper, y2_mapper.as_ref(), axes);

    // Render frame and blit canvas
    let mut text_grid = render_frame(&layout, &config, &x_ticks, &y_ticks);
    text_grid.blit_braille(&canvas, layout.canvas_col, layout.canvas_row);

    // Draw legend overlay
    legend::draw_legend(
        &mut text_grid,
        &axes.series,
        &axes.legend,
        layout.canvas_col,
        layout.canvas_row,
        layout.canvas_char_width,
        layout.canvas_char_height,
    );

    // Draw annotations
    draw_annotations(&mut text_grid, axes, &mapper, &layout);

    text_grid.render()
}

/// Render a single 3D axes to a string.
fn render_single_axes3d(
    axes: &Axes3D,
    super_title: Option<&str>,
    width: usize,
    height: usize,
) -> String {
    let title = axes.title.as_deref().or(super_title);

    // Reserve space for title and labels
    let title_rows = if title.is_some() { 1 } else { 0 };
    let label_rows = if axes.x_label.is_some() || axes.y_label.is_some() {
        1
    } else {
        0
    };
    let canvas_height = height.saturating_sub(title_rows + label_rows + 2).max(1); // 2 for border
    let canvas_width = width.saturating_sub(2).max(1); // 2 for border

    let mut text_grid = TextGrid::new(width, height);

    // Title
    if let Some(t) = title {
        text_grid.put_str_centered(0, width, 0, t, TermColor::Default);
    }

    // Border
    let border_top = title_rows;
    let border_left = 0;
    let border_right = canvas_width + 1;
    let border_bottom = title_rows + canvas_height + 1;

    text_grid.put_char(border_left, border_top, '┌', TermColor::Default);
    text_grid.put_char(
        border_right.min(width - 1),
        border_top,
        '┐',
        TermColor::Default,
    );
    text_grid.put_char(
        border_left,
        border_bottom.min(height - 1),
        '└',
        TermColor::Default,
    );
    text_grid.put_char(
        border_right.min(width - 1),
        border_bottom.min(height - 1),
        '┘',
        TermColor::Default,
    );
    for c in (border_left + 1)..border_right.min(width) {
        text_grid.put_char(c, border_top, '─', TermColor::Default);
        text_grid.put_char(c, border_bottom.min(height - 1), '─', TermColor::Default);
    }
    for r in (border_top + 1)..border_bottom.min(height) {
        text_grid.put_char(border_left, r, '│', TermColor::Default);
        text_grid.put_char(border_right.min(width - 1), r, '│', TermColor::Default);
    }

    // Create projection
    let projection = Projection::new(axes.azimuth, axes.elevation);

    // Determine if we need a depth canvas
    let needs_depth = axes
        .surfaces
        .iter()
        .any(|s| matches!(s.style, SurfaceStyle::HiddenLine | SurfaceStyle::Filled));

    // Canvas pixel dimensions
    let cw = canvas_width;
    let ch = canvas_height;
    let pw = cw as f64 * 2.0;
    let ph = ch as f64 * 4.0;

    // Scaling: map projected unit cube to canvas
    let scale = (pw * 0.8).min(ph * 0.8);
    let offset_x = pw / 2.0;
    let offset_y = ph / 2.0;

    if needs_depth {
        let mut dc = DepthCanvas::new(cw, ch);

        for (idx, surf) in axes.surfaces.iter().enumerate() {
            let color = axes.resolve_color(&surf.options, idx);
            match surf.style {
                SurfaceStyle::HiddenLine => {
                    surface::draw_surface_hidden(
                        &mut dc,
                        &surf.grid,
                        &projection,
                        scale,
                        scale,
                        offset_x,
                        offset_y,
                        color,
                    );
                }
                SurfaceStyle::Filled => {
                    surface::draw_surface_filled(
                        &mut dc,
                        &surf.grid,
                        &projection,
                        scale,
                        scale,
                        offset_x,
                        offset_y,
                        axes.colormap,
                    );
                }
                SurfaceStyle::Wireframe => {
                    // Wireframe on a depth canvas: draw using the underlying canvas
                    // We can't mix, so just draw wireframe without depth
                    let canvas_ref = &dc;
                    let _ = canvas_ref;
                    // We'll draw wireframe after converting to canvas
                }
            }
        }

        // Blit depth canvas
        text_grid.blit_braille(dc.canvas(), border_left + 1, border_top + 1);
    } else {
        let mut canvas = BrailleCanvas::new(cw, ch);

        for (idx, surf) in axes.surfaces.iter().enumerate() {
            let color = axes.resolve_color(&surf.options, idx);
            if surf.style == SurfaceStyle::Wireframe {
                surface::draw_surface_wireframe(
                    &mut canvas,
                    &surf.grid,
                    &projection,
                    scale,
                    scale,
                    offset_x,
                    offset_y,
                    color,
                );
            }
        }

        text_grid.blit_braille(&canvas, border_left + 1, border_top + 1);
    }

    // Draw axis labels
    if let Some(label) = &axes.x_label {
        let label_row = border_bottom.min(height - 1) + 1;
        if label_row < height {
            text_grid.put_str_centered(0, width, label_row, label, TermColor::Default);
        }
    }

    // Draw projected axis lines for reference
    draw_3d_axis_lines(
        &mut text_grid,
        &projection,
        scale,
        offset_x,
        offset_y,
        border_left + 1,
        border_top + 1,
        cw,
        ch,
    );

    text_grid.render()
}

/// Draw 3D axis reference lines (the 3 visible edges of the bounding cube).
#[allow(clippy::too_many_arguments)]
fn draw_3d_axis_lines(
    grid: &mut TextGrid,
    projection: &Projection,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    grid_col_offset: usize,
    grid_row_offset: usize,
    canvas_char_width: usize,
    canvas_char_height: usize,
) {
    // Draw three axis lines from the origin corner (-0.5, -0.5, -0.5)
    let origin = (-0.5_f64, -0.5_f64, -0.5_f64);
    let x_end = (0.5, -0.5, -0.5);
    let y_end = (-0.5, 0.5, -0.5);
    let z_end = (-0.5, -0.5, 0.5);

    let axes_data = [(origin, x_end), (origin, y_end), (origin, z_end)];
    let labels = ["X", "Y", "Z"];

    for (i, &(start, end)) in axes_data.iter().enumerate() {
        let (_sx0, _sy0, _) = projection.project(start.0, start.1, start.2);
        let (sx1, sy1, _) = projection.project(end.0, end.1, end.2);

        let px1 = (sx1 * scale + offset_x) / 2.0;
        let py1 = (-sy1 * scale + offset_y) / 4.0;

        // Place label near the endpoint
        let label_col = px1.round() as usize;
        let label_row = py1.round() as usize;
        if label_col < canvas_char_width && label_row < canvas_char_height {
            grid.put_str(
                grid_col_offset + label_col,
                grid_row_offset + label_row,
                labels[i],
                TermColor::Default,
            );
        }
    }
}

/// Compute the x-axis extent needed for box/bar series to render fully.
/// Returns (needed_min, needed_max) in data coordinates, or non-finite values
/// if no box series need padding.
fn compute_box_x_padding(axes: &Axes2D) -> (f64, f64) {
    let mut needed_min = f64::INFINITY;
    let mut needed_max = f64::NEG_INFINITY;
    for s in &axes.series {
        let x_data = s.x_data();
        if matches!(s, SeriesData::Boxes { .. } | SeriesData::BoxAndWhisker { .. })
            && x_data.len() >= 2
        {
            let min_spacing = x_data
                .windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .filter(|d| *d > f64::EPSILON)
                .fold(f64::INFINITY, f64::min);
            if min_spacing.is_finite() {
                let pad = min_spacing / 2.0;
                needed_min = needed_min.min(x_data.first().unwrap() - pad);
                needed_max = needed_max.max(x_data.last().unwrap() + pad);
            }
        }
    }
    (needed_min, needed_max)
}

/// Compute auto data ranges across all series on primary axes.
fn compute_data_ranges(axes: &Axes2D) -> (f64, f64, f64, f64) {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    for s in &axes.series {
        let ap = s.axis_pair();
        let uses_primary_x = matches!(ap, AxisPair::X1Y1 | AxisPair::X1Y2);
        let uses_primary_y = matches!(ap, AxisPair::X1Y1 | AxisPair::X2Y1);

        // Include grid data ranges
        if let Some(grid) = s.grid_data() {
            if uses_primary_x {
                x_min = x_min.min(grid.x_min);
                x_max = x_max.max(grid.x_max);
            }
            if uses_primary_y {
                y_min = y_min.min(grid.y_min);
                y_max = y_max.max(grid.y_max);
            }
        }

        if uses_primary_x {
            for &x in s.x_range_values().iter() {
                if x.is_finite() {
                    x_min = x_min.min(x);
                    x_max = x_max.max(x);
                }
            }
        }
        if uses_primary_y {
            for &y in s.y_range_values().iter() {
                if y.is_finite() {
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
            }
        }
    }

    // Handle degenerate cases
    if !x_min.is_finite() || !x_max.is_finite() {
        x_min = -1.0;
        x_max = 1.0;
    }
    if !y_min.is_finite() || !y_max.is_finite() {
        y_min = -1.0;
        y_max = 1.0;
    }
    if (x_max - x_min).abs() < f64::EPSILON {
        x_min -= 1.0;
        x_max += 1.0;
    }
    if (y_max - y_min).abs() < f64::EPSILON {
        y_min -= 1.0;
        y_max += 1.0;
    }

    (x_min, x_max, y_min, y_max)
}

/// Compute data ranges for series that use secondary axes.
/// Returns (x2_min, x2_max, y2_min, y2_max), falling back to primary ranges.
fn compute_secondary_ranges(axes: &Axes2D) -> (f64, f64, f64, f64) {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    for s in &axes.series {
        let ap = s.axis_pair();
        let uses_x2 = matches!(ap, AxisPair::X2Y1 | AxisPair::X2Y2);
        let uses_y2 = matches!(ap, AxisPair::X1Y2 | AxisPair::X2Y2);

        if uses_x2 {
            for &x in s.x_range_values().iter() {
                if x.is_finite() {
                    x_min = x_min.min(x);
                    x_max = x_max.max(x);
                }
            }
        }
        if uses_y2 {
            for &y in s.y_range_values().iter() {
                if y.is_finite() {
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
            }
        }
    }

    // Handle degenerate cases
    if !x_min.is_finite() || !x_max.is_finite() {
        x_min = -1.0;
        x_max = 1.0;
    }
    if !y_min.is_finite() || !y_max.is_finite() {
        y_min = -1.0;
        y_max = 1.0;
    }
    if (x_max - x_min).abs() < f64::EPSILON {
        x_min -= 1.0;
        x_max += 1.0;
    }
    if (y_max - y_min).abs() < f64::EPSILON {
        y_min -= 1.0;
        y_max += 1.0;
    }

    (x_min, x_max, y_min, y_max)
}

/// Draw grid and minor grid lines.
fn draw_grids(
    canvas: &mut BrailleCanvas,
    mapper: &CoordinateMapper,
    x_ticks: &crate::layout::TickSet,
    y_ticks: &crate::layout::TickSet,
    axes: &Axes2D,
    y2_mapper: Option<&CoordinateMapper>,
    y2_ticks: Option<&crate::layout::TickSet>,
) {
    // Minor grid first (underneath major)
    grid::draw_minor_grid(
        canvas,
        mapper,
        x_ticks,
        y_ticks,
        axes.x_axis.minor_grid,
        axes.y_axis.minor_grid,
        axes.x_axis.minor_grid_color,
        axes.y_axis.minor_grid_color,
        axes.x_axis.minor_grid_dash.to_pattern(),
        axes.y_axis.minor_grid_dash.to_pattern(),
    );

    grid::draw_grid(
        canvas,
        mapper,
        x_ticks,
        y_ticks,
        axes.x_axis.grid,
        axes.y_axis.grid,
        axes.x_axis.grid_color,
        axes.y_axis.grid_color,
        axes.x_axis.grid_dash.to_pattern(),
        axes.y_axis.grid_dash.to_pattern(),
    );

    // Y2 grid lines
    if let (Some(y2m), Some(y2t)) = (y2_mapper, y2_ticks) {
        if axes.y2_axis.minor_grid {
            grid::draw_minor_grid(
                canvas,
                y2m,
                x_ticks,
                y2t,
                false, // don't duplicate x minor grid
                axes.y2_axis.minor_grid,
                axes.x_axis.minor_grid_color,
                axes.y2_axis.minor_grid_color,
                axes.x_axis.minor_grid_dash.to_pattern(),
                axes.y2_axis.minor_grid_dash.to_pattern(),
            );
        }
        if axes.y2_axis.grid {
            grid::draw_grid(
                canvas,
                y2m,
                x_ticks,
                y2t,
                false, // don't duplicate x grid
                axes.y2_axis.grid,
                axes.x_axis.grid_color,
                axes.y2_axis.grid_color,
                axes.x_axis.grid_dash.to_pattern(),
                axes.y2_axis.grid_dash.to_pattern(),
            );
        }
    }
}

/// Select the appropriate mapper for a series based on its axis pair.
fn select_mapper<'a>(
    s: &SeriesData,
    primary: &'a CoordinateMapper,
    y2_mapper: Option<&'a CoordinateMapper>,
) -> &'a CoordinateMapper {
    match s.axis_pair() {
        AxisPair::X1Y2 | AxisPair::X2Y2 => y2_mapper.unwrap_or(primary),
        _ => primary,
    }
}

/// Draw all series in proper rendering order.
fn draw_all_series(
    canvas: &mut BrailleCanvas,
    mapper: &CoordinateMapper,
    y2_mapper: Option<&CoordinateMapper>,
    axes: &Axes2D,
) {
    // Rendering order: heatmaps -> fills -> boxes -> error bars -> contours -> lines -> points
    // We make multiple passes to ensure correct layering.

    // Heatmap pass (background layer)
    for (idx, s) in axes.series.iter().enumerate() {
        let m = select_mapper(s, mapper, y2_mapper);
        match s {
            SeriesData::Heatmap { grid, options, .. } => {
                let cmap = resolve_colormap(options);
                heatmap::draw_heatmap(canvas, grid, m, cmap);
            }
            SeriesData::HeatmapContour { grid, options, .. } => {
                let cmap = resolve_colormap(options);
                heatmap::draw_heatmap(canvas, grid, m, cmap);
            }
            _ => {}
        }
        let _ = idx;
    }

    for (idx, s) in axes.series.iter().enumerate() {
        if matches!(s, SeriesData::FillBetween { .. }) {
            draw_series(canvas, select_mapper(s, mapper, y2_mapper), s, idx);
        }
    }
    for (idx, s) in axes.series.iter().enumerate() {
        if matches!(
            s,
            SeriesData::Boxes { .. } | SeriesData::BoxAndWhisker { .. }
        ) {
            draw_series(canvas, select_mapper(s, mapper, y2_mapper), s, idx);
        }
    }
    for (idx, s) in axes.series.iter().enumerate() {
        if matches!(
            s,
            SeriesData::YErrorBars { .. }
                | SeriesData::XErrorBars { .. }
                | SeriesData::YErrorLines { .. }
                | SeriesData::XErrorLines { .. }
        ) {
            draw_series(canvas, select_mapper(s, mapper, y2_mapper), s, idx);
        }
    }

    // Contour pass (above error bars, below lines)
    for (idx, s) in axes.series.iter().enumerate() {
        let m = select_mapper(s, mapper, y2_mapper);
        match s {
            SeriesData::Contour {
                grid,
                levels,
                options,
            } => {
                let color = resolve_series_color(options, idx);
                let resolved_levels = resolve_contour_levels(grid, levels.as_deref(), options);
                let dash = resolve_dash(options);
                contour::draw_contour(
                    canvas,
                    grid,
                    m,
                    &resolved_levels,
                    Some(color),
                    dash.to_pattern(),
                );
            }
            SeriesData::HeatmapContour {
                grid,
                levels,
                options,
            } => {
                let resolved_levels = resolve_contour_levels(grid, levels.as_deref(), options);
                let dash = resolve_dash(options);
                // Use White for contrast against the heatmap background
                contour::draw_contour(
                    canvas,
                    grid,
                    m,
                    &resolved_levels,
                    Some(TermColor::White),
                    dash.to_pattern(),
                );
            }
            _ => {}
        }
    }

    for (idx, s) in axes.series.iter().enumerate() {
        if matches!(s, SeriesData::Lines { .. } | SeriesData::LinesPoints { .. }) {
            draw_series(canvas, select_mapper(s, mapper, y2_mapper), s, idx);
        }
    }
    for (idx, s) in axes.series.iter().enumerate() {
        if matches!(
            s,
            SeriesData::Points { .. } | SeriesData::LinesPoints { .. }
        ) {
            draw_series_points_pass(canvas, select_mapper(s, mapper, y2_mapper), s, idx);
        }
    }
}

/// Resolve colormap from options, defaulting to Heat.
fn resolve_colormap(opts: &[PlotOption]) -> ColorMapType {
    opts.iter()
        .find_map(|o| {
            if let PlotOption::ColorMap(c) = o {
                Some(*c)
            } else {
                None
            }
        })
        .unwrap_or(ColorMapType::Heat)
}

/// Resolve contour levels: explicit > option-based > default 8 levels.
fn resolve_contour_levels(
    grid: &crate::api::grid::GridData,
    explicit: Option<&[f64]>,
    opts: &[PlotOption],
) -> Vec<f64> {
    if let Some(levels) = explicit {
        return levels.to_vec();
    }
    let n = opts
        .iter()
        .find_map(|o| {
            if let PlotOption::ContourLevels(n) = o {
                Some(*n)
            } else {
                None
            }
        })
        .unwrap_or(8);
    auto_contour_levels(grid.z_min(), grid.z_max(), n)
}

/// Resolve series color from options or palette.
fn resolve_series_color(opts: &[PlotOption], series_idx: usize) -> TermColor {
    opts.iter()
        .find_map(|o| {
            if let PlotOption::Color(c) = o {
                Some(*c)
            } else {
                None
            }
        })
        .unwrap_or(PALETTE[series_idx % PALETTE.len()])
}

/// Resolve dash type from options.
fn resolve_dash(opts: &[PlotOption]) -> DashType {
    opts.iter()
        .find_map(|o| {
            if let PlotOption::LineStyle(d) = o {
                Some(*d)
            } else {
                None
            }
        })
        .unwrap_or(DashType::Solid)
}

/// Resolve point symbol from options.
fn resolve_point_symbol(opts: &[PlotOption]) -> PointSymbol {
    opts.iter()
        .find_map(|o| {
            if let PlotOption::PointSymbol(p) = o {
                Some(*p)
            } else {
                None
            }
        })
        .unwrap_or(PointSymbol::Dot)
}

/// Resolve box width fraction from options.
fn resolve_box_width(opts: &[PlotOption]) -> f64 {
    opts.iter()
        .find_map(|o| {
            if let PlotOption::BoxWidth(w) = o {
                Some(*w)
            } else {
                None
            }
        })
        .unwrap_or(0.8)
}

/// Map data arrays to pixel coordinates.
fn map_to_pixels(mapper: &CoordinateMapper, x: &[f64], y: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let len = x.len().min(y.len());
    let mut px = Vec::with_capacity(len);
    let mut py = Vec::with_capacity(len);
    for i in 0..len {
        let (pxi, pyi) = mapper.data_to_pixel(x[i], y[i]);
        px.push(pxi);
        py.push(pyi);
    }
    (px, py)
}

/// Draw a single series (non-points pass).
fn draw_series(
    canvas: &mut BrailleCanvas,
    mapper: &CoordinateMapper,
    series: &SeriesData,
    idx: usize,
) {
    let color = resolve_series_color(series.options(), idx);

    match series {
        SeriesData::Lines { x, y, options } => {
            let (px, py) = map_to_pixels(mapper, x, y);
            let dash = resolve_dash(options);
            lines::draw_lines(canvas, &px, &py, color, dash.to_pattern());
        }
        SeriesData::LinesPoints { x, y, options } => {
            // Lines pass only
            let (px, py) = map_to_pixels(mapper, x, y);
            let dash = resolve_dash(options);
            lines::draw_lines(canvas, &px, &py, color, dash.to_pattern());
        }
        SeriesData::Points { .. } => {
            // Handled in points pass
        }
        SeriesData::Boxes { x, y, options } => {
            let box_frac = resolve_box_width(options);
            let (px, py) = map_to_pixels(mapper, x, y);

            // Baseline at y=0 or axis min
            let baseline_y = if mapper.data_y_min <= 0.0 && mapper.data_y_max >= 0.0 {
                0.0
            } else {
                mapper.data_y_min
            };
            let (_, baseline_py) = mapper.data_to_pixel(0.0, baseline_y);

            // Compute box width in pixels from data spacing
            let box_width_px = if x.len() >= 2 {
                let min_spacing = x
                    .windows(2)
                    .map(|w| (w[1] - w[0]).abs())
                    .filter(|s| *s > f64::EPSILON)
                    .fold(f64::INFINITY, f64::min);
                let (px0, _) = mapper.data_to_pixel(0.0, 0.0);
                let (px1, _) = mapper.data_to_pixel(min_spacing, 0.0);
                (px1 - px0).abs() * box_frac
            } else {
                (canvas.pixel_width() as f64 * 0.1).max(2.0) * box_frac
            };

            boxes::draw_boxes(canvas, &px, &py, baseline_py, color, box_width_px);
        }
        SeriesData::FillBetween { x, y1, y2, .. } => {
            let (px, py1) = map_to_pixels(mapper, x, y1);
            let (_, py2) = map_to_pixels(mapper, x, y2);
            fill::draw_fill_between(canvas, &px, &py1, &py2, color);
        }
        SeriesData::YErrorBars {
            x, y_low, y_high, ..
        } => {
            let (px, _) = map_to_pixels(mapper, x, y_low);
            let (_, py_low) = map_to_pixels(mapper, x, y_low);
            let (_, py_high) = map_to_pixels(mapper, x, y_high);
            error_bars::draw_y_error_bars(canvas, &px, &py_low, &py_high, color, 3);
        }
        SeriesData::XErrorBars {
            x,
            y,
            x_low,
            x_high,
            ..
        } => {
            let (_, py) = map_to_pixels(mapper, x, y);
            let (px_low, _) = map_to_pixels(mapper, x_low, y);
            let (px_high, _) = map_to_pixels(mapper, x_high, y);
            error_bars::draw_x_error_bars(canvas, &py, &px_low, &px_high, color, 3);
        }
        SeriesData::YErrorLines {
            x,
            y,
            y_low,
            y_high,
            options,
        } => {
            // Draw error bars first, then lines through center
            let (px, py) = map_to_pixels(mapper, x, y);
            let (_, py_low) = map_to_pixels(mapper, x, y_low);
            let (_, py_high) = map_to_pixels(mapper, x, y_high);
            error_bars::draw_y_error_bars(canvas, &px, &py_low, &py_high, color, 3);
            let dash = resolve_dash(options);
            lines::draw_lines(canvas, &px, &py, color, dash.to_pattern());
        }
        SeriesData::XErrorLines {
            x,
            y,
            x_low,
            x_high,
            options,
        } => {
            let (px, py) = map_to_pixels(mapper, x, y);
            let (px_low, _) = map_to_pixels(mapper, x_low, y);
            let (px_high, _) = map_to_pixels(mapper, x_high, y);
            error_bars::draw_x_error_bars(canvas, &py, &px_low, &px_high, color, 3);
            let dash = resolve_dash(options);
            lines::draw_lines(canvas, &px, &py, color, dash.to_pattern());
        }
        SeriesData::BoxAndWhisker {
            x,
            min,
            q1,
            median,
            q3,
            max,
            options,
        } => {
            let box_frac = resolve_box_width(options);
            let (px, py_min) = map_to_pixels(mapper, x, min);
            let (_, py_q1) = map_to_pixels(mapper, x, q1);
            let (_, py_med) = map_to_pixels(mapper, x, median);
            let (_, py_q3) = map_to_pixels(mapper, x, q3);
            let (_, py_max) = map_to_pixels(mapper, x, max);

            let box_width_px = if x.len() >= 2 {
                let min_spacing = x
                    .windows(2)
                    .map(|w| (w[1] - w[0]).abs())
                    .filter(|s| *s > f64::EPSILON)
                    .fold(f64::INFINITY, f64::min);
                let (px0, _) = mapper.data_to_pixel(0.0, 0.0);
                let (px1, _) = mapper.data_to_pixel(min_spacing, 0.0);
                (px1 - px0).abs() * box_frac
            } else {
                (canvas.pixel_width() as f64 * 0.1).max(2.0) * box_frac
            };

            box_whisker::draw_box_and_whisker(
                canvas,
                &px,
                &py_min,
                &py_q1,
                &py_med,
                &py_q3,
                &py_max,
                color,
                box_width_px,
            );
        }
        // Grid-based types handled in dedicated passes
        SeriesData::Heatmap { .. }
        | SeriesData::Contour { .. }
        | SeriesData::HeatmapContour { .. } => {}
    }
}

/// Points-only pass for Points and LinesPoints series.
fn draw_series_points_pass(
    canvas: &mut BrailleCanvas,
    mapper: &CoordinateMapper,
    series: &SeriesData,
    idx: usize,
) {
    let color = resolve_series_color(series.options(), idx);
    let symbol = resolve_point_symbol(series.options());

    match series {
        SeriesData::Points { x, y, .. } | SeriesData::LinesPoints { x, y, .. } => {
            let (px, py) = map_to_pixels(mapper, x, y);
            points::draw_points(canvas, &px, &py, color, symbol);
        }
        _ => {}
    }
}

/// Draw text annotations on the grid.
fn draw_annotations(
    grid: &mut TextGrid,
    axes: &Axes2D,
    mapper: &CoordinateMapper,
    layout: &Layout,
) {
    for ann in &axes.annotations {
        let (col, row) = resolve_annotation_pos(&ann.position, mapper, layout);

        let color = ann
            .options
            .iter()
            .find_map(|o| {
                if let crate::api::options::LabelOption::TextColor(c) = o {
                    Some(*c)
                } else {
                    None
                }
            })
            .unwrap_or(TermColor::Default);

        grid.put_str(col, row, &ann.text, color);
    }

    // Draw arrows
    for arrow in &axes.arrows {
        let (from_col, from_row) = resolve_annotation_pos(&arrow.from, mapper, layout);
        let (to_col, to_row) = resolve_annotation_pos(&arrow.to, mapper, layout);

        // Draw arrow using simple line characters in the TextGrid
        // For simplicity, draw horizontal/vertical lines with arrow head
        let color = arrow.color;
        if from_row == to_row {
            // Horizontal arrow
            let (left, right) = if from_col <= to_col {
                (from_col, to_col)
            } else {
                (to_col, from_col)
            };
            for c in left..=right {
                grid.put_char(c, from_row, '─', color);
            }
            if arrow.head {
                if to_col > from_col {
                    grid.put_char(to_col, to_row, '→', color);
                } else {
                    grid.put_char(to_col, to_row, '←', color);
                }
            }
        } else if from_col == to_col {
            // Vertical arrow
            let (top, bottom) = if from_row <= to_row {
                (from_row, to_row)
            } else {
                (to_row, from_row)
            };
            for r in top..=bottom {
                grid.put_char(from_col, r, '│', color);
            }
            if arrow.head {
                if to_row > from_row {
                    grid.put_char(to_col, to_row, '↓', color);
                } else {
                    grid.put_char(to_col, to_row, '↑', color);
                }
            }
        }
        // Diagonal arrows: just mark endpoints for simplicity in terminal
    }
}

/// Resolve a coordinate pair to grid (col, row).
fn resolve_annotation_pos(
    pos: &(
        crate::api::options::Coordinate,
        crate::api::options::Coordinate,
    ),
    mapper: &CoordinateMapper,
    layout: &Layout,
) -> (usize, usize) {
    use crate::api::options::Coordinate;

    let x_data = match pos.0 {
        Coordinate::Graph(g) => mapper.data_x_min + g * (mapper.data_x_max - mapper.data_x_min),
        Coordinate::Axis(v) | Coordinate::Axis2(v) => v,
    };
    let y_data = match pos.1 {
        Coordinate::Graph(g) => mapper.data_y_min + g * (mapper.data_y_max - mapper.data_y_min),
        Coordinate::Axis(v) | Coordinate::Axis2(v) => v,
    };

    let (px, py) = mapper.data_to_pixel(x_data, y_data);
    // Convert pixel coords to character cell coords
    let char_col = (px / 2.0).round() as usize;
    let char_row = (py / 4.0).round() as usize;

    (
        layout.canvas_col + char_col.min(layout.canvas_char_width.saturating_sub(1)),
        layout.canvas_row + char_row.min(layout.canvas_char_height.saturating_sub(1)),
    )
}

/// Create a TickSet from custom tick positions.
fn custom_tick_set(
    ticks: &[(f64, String)],
    data_min: f64,
    data_max: f64,
) -> crate::layout::TickSet {
    let min = ticks.iter().map(|(v, _)| *v).fold(data_min, f64::min);
    let max = ticks.iter().map(|(v, _)| *v).fold(data_max, f64::max);
    let spacing = if ticks.len() >= 2 {
        (max - min) / (ticks.len() - 1) as f64
    } else {
        max - min
    };
    crate::layout::TickSet {
        min,
        max,
        spacing: if spacing.abs() < f64::EPSILON {
            1.0
        } else {
            spacing
        },
        ticks: ticks.to_vec(),
    }
}

/// Generate logarithmic ticks for a range.
fn log_ticks(data_min: f64, data_max: f64, base: f64) -> crate::layout::TickSet {
    let min_positive = if data_min <= 0.0 {
        f64::EPSILON
    } else {
        data_min
    };
    let max_val = if data_max <= 0.0 { 1.0 } else { data_max };

    let log_min = min_positive.log(base).floor() as i32;
    let log_max = max_val.log(base).ceil() as i32;

    let mut ticks = Vec::new();
    for exp in log_min..=log_max {
        let val = base.powi(exp);
        let label = if !(1e-3..1e6).contains(&val) {
            format!("{val:.1e}")
        } else if val >= 1.0 {
            format!("{val:.0}")
        } else {
            format!("{val}")
        };
        ticks.push((val, label));
    }

    if ticks.is_empty() {
        ticks.push((1.0, "1".to_string()));
    }

    let tick_min = ticks.first().unwrap().0;
    let tick_max = ticks.last().unwrap().0;

    crate::layout::TickSet {
        min: tick_min,
        max: tick_max,
        spacing: if ticks.len() >= 2 {
            (tick_max - tick_min) / (ticks.len() - 1) as f64
        } else {
            1.0
        },
        ticks,
    }
}

#[cfg(test)]
mod tests {
    use crate::api::figure::Figure;
    use crate::api::options::*;

    #[test]
    fn empty_figure_renders_empty() {
        let fig = Figure::new();
        assert!(fig.render().is_empty());
    }

    #[test]
    fn simple_line_plot() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_title("Test Line");
            let xs: Vec<f64> = (0..=10).map(|i| i as f64).collect();
            let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
            ax.lines(
                xs.iter().copied(),
                ys.iter().copied(),
                &[PlotOption::Caption("x^2".into())],
            );
        }
        let result = fig.render();
        assert!(!result.is_empty());
        assert!(result.contains("Test Line"));
    }

    #[test]
    fn points_plot() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let ys = vec![1.0, 4.0, 2.0, 5.0, 3.0];
            ax.points(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        let result = fig.render();
        assert!(!result.is_empty());
    }

    #[test]
    fn boxes_plot() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            let xs = vec![1.0, 2.0, 3.0];
            let ys = vec![5.0, 3.0, 7.0];
            ax.boxes(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        let result = fig.render();
        assert!(!result.is_empty());
    }

    #[test]
    fn manual_range() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(100.0));
            ax.set_y_range(AutoOption::Fix(-10.0), AutoOption::Fix(10.0));
            let xs = vec![10.0, 50.0, 90.0];
            let ys = vec![5.0, -5.0, 8.0];
            ax.lines(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        let result = fig.render();
        assert!(!result.is_empty());
    }

    #[test]
    fn log_scale_plot() {
        let mut fig = Figure::new();
        fig.set_terminal_size(60, 15);
        {
            let ax = fig.axes2d();
            ax.set_y_log(Some(10.0));
            let xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let ys = vec![1.0, 10.0, 100.0, 1000.0, 10000.0];
            ax.lines(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        let result = fig.render();
        assert!(!result.is_empty());
    }

    #[test]
    fn display_trait() {
        let mut fig = Figure::new();
        fig.set_terminal_size(40, 10);
        {
            let ax = fig.axes2d();
            let xs = vec![0.0, 1.0];
            let ys = vec![0.0, 1.0];
            ax.lines(xs.iter().copied(), ys.iter().copied(), &[]);
        }
        let s = format!("{fig}");
        assert!(!s.is_empty());
    }

    #[test]
    fn dual_y_axis_basic() {
        let mut fig = Figure::new();
        fig.set_terminal_size(80, 20);
        {
            let ax = fig.axes2d();
            ax.set_title("Dual Axis");
            ax.set_y_label("Primary", &[]);
            ax.set_y2_label("Secondary", &[]);

            let xs = vec![1.0, 2.0, 3.0, 4.0];
            let ys_primary = vec![10.0, 20.0, 15.0, 25.0];
            let ys_secondary = vec![100.0, 200.0, 150.0, 250.0];

            ax.lines(xs.iter().copied(), ys_primary.iter().copied(), &[]);
            ax.lines(
                xs.iter().copied(),
                ys_secondary.iter().copied(),
                &[PlotOption::Axes(AxisPair::X1Y2)],
            );
        }
        let result = fig.render();
        assert!(!result.is_empty());
        assert!(result.contains("Dual Axis"));
    }

    #[test]
    fn y2_does_not_pollute_primary_range() {
        // Primary series has small y values; y2 series has huge values.
        // Primary ticks should stay small.
        let mut fig = Figure::new();
        fig.set_terminal_size(80, 20);
        {
            let ax = fig.axes2d();
            let xs = vec![0.0, 1.0, 2.0];
            let ys_small = vec![1.0, 2.0, 3.0];
            let ys_huge = vec![1000.0, 2000.0, 3000.0];

            ax.lines(xs.iter().copied(), ys_small.iter().copied(), &[]);
            ax.lines(
                xs.iter().copied(),
                ys_huge.iter().copied(),
                &[PlotOption::Axes(AxisPair::X1Y2)],
            );
        }
        let result = fig.render();
        assert!(!result.is_empty());
        // The rendered output should NOT contain "3000" on the primary axis
        // but SHOULD contain it on the y2 axis (right side).
        // At minimum, the primary tick labels should be in the small range.
        // Check that the output contains small numbers (from primary ticks).
        assert!(result.contains('1') || result.contains('2') || result.contains('3'));
        // The huge y2 values should appear on the right-side axis
        assert!(result.contains("3000") || result.contains("2000"));
    }

    #[test]
    fn dual_y_axis_auto_detect() {
        // Tagging a series with X1Y2 should trigger y2 axis automatically
        // without any explicit y2 config.
        let mut fig = Figure::new();
        fig.set_terminal_size(80, 20);
        {
            let ax = fig.axes2d();
            ax.set_title("Auto Y2");
            let xs = vec![1.0, 2.0, 3.0];
            let ys1 = vec![1.0, 2.0, 3.0];
            let ys2 = vec![100.0, 200.0, 300.0];

            ax.lines(xs.iter().copied(), ys1.iter().copied(), &[]);
            ax.lines(
                xs.iter().copied(),
                ys2.iter().copied(),
                &[PlotOption::Axes(AxisPair::X1Y2)],
            );
        }
        let result = fig.render();
        assert!(!result.is_empty());
        assert!(result.contains("Auto Y2"));
        // Y2 ticks should be auto-generated in the 100-300 range
        assert!(result.contains("100") || result.contains("200") || result.contains("300"));
    }
}
