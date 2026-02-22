pub mod canvas;
pub mod layout;
pub mod transform;

use canvas::{BrailleCanvas, PALETTE};
use layout::{LayoutConfig, compute_layout, generate_ticks, render_frame};
use transform::{CoordinateMapper, clip_line};

/// Quick one-shot plot of a line series. Returns the rendered string.
///
/// Orchestrates the full pipeline: data range computation, tick generation,
/// layout, coordinate mapping, Braille rendering, and frame compositing.
pub fn quick_plot(
    x_data: &[f64],
    y_data: &[f64],
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
    width: usize,
    height: usize,
) -> String {
    quick_plot_multi(&[(x_data, y_data)], title, x_label, y_label, width, height)
}

/// Quick one-shot plot of multiple line series. Returns the rendered string.
pub fn quick_plot_multi(
    series: &[(&[f64], &[f64])],
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
    width: usize,
    height: usize,
) -> String {
    // Compute data ranges across all series
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );

    for &(xs, ys) in series {
        for &x in xs {
            if x.is_finite() {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }
        for &y in ys {
            if y.is_finite() {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
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

    // Add 5% padding
    let x_pad = (x_max - x_min) * 0.05;
    let y_pad = (y_max - y_min) * 0.05;
    x_min -= x_pad;
    x_max += x_pad;
    y_min -= y_pad;
    y_max += y_pad;

    // Generate ticks
    let x_ticks = generate_ticks(x_min, x_max, 6);
    let y_ticks = generate_ticks(y_min, y_max, 5);

    // Compute layout
    let config = LayoutConfig {
        total_width: width,
        total_height: height,
        title: title.map(String::from),
        x_label: x_label.map(String::from),
        y_label: y_label.map(String::from),
    };
    let layout = compute_layout(&config, &x_ticks, &y_ticks);

    // Use tick-rounded ranges for the mapper
    let mapper = CoordinateMapper::new(
        x_ticks.min,
        x_ticks.max,
        y_ticks.min,
        y_ticks.max,
        layout.canvas_char_width * 2,
        layout.canvas_char_height * 4,
    );

    // Create canvas and draw
    let mut canvas = BrailleCanvas::new(layout.canvas_char_width, layout.canvas_char_height);

    let pw = canvas.pixel_width() as f64;
    let ph = canvas.pixel_height() as f64;

    for (series_idx, &(xs, ys)) in series.iter().enumerate() {
        let color = PALETTE[series_idx % PALETTE.len()];
        let len = xs.len().min(ys.len());

        for i in 1..len {
            let x0 = xs[i - 1];
            let y0 = ys[i - 1];
            let x1 = xs[i];
            let y1 = ys[i];

            if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
                continue;
            }

            let (px0, py0) = mapper.data_to_pixel(x0, y0);
            let (px1, py1) = mapper.data_to_pixel(x1, y1);

            // Clip to canvas pixel bounds
            if let Some((cx0, cy0, cx1, cy1)) =
                clip_line(px0, py0, px1, py1, 0.0, 0.0, pw - 1.0, ph - 1.0)
            {
                canvas.draw_line_solid(
                    cx0.round() as i32,
                    cy0.round() as i32,
                    cx1.round() as i32,
                    cy1.round() as i32,
                    color,
                );
            }
        }
    }

    // Render frame and blit canvas
    let mut grid = render_frame(&layout, &config, &x_ticks, &y_ticks, &mapper);
    grid.blit_braille(&canvas, layout.canvas_col, layout.canvas_row);

    grid.render()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_plot_produces_output() {
        let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
        let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
        let result = quick_plot(&xs, &ys, Some("Parabola"), Some("x"), Some("y"), 80, 24);
        assert!(!result.is_empty());
    }

    #[test]
    fn output_contains_title() {
        let xs = vec![0.0, 1.0, 2.0];
        let ys = vec![0.0, 1.0, 4.0];
        let result = quick_plot(&xs, &ys, Some("MyTitle"), None, None, 60, 15);
        assert!(result.contains("MyTitle"));
    }

    #[test]
    fn output_contains_border_chars() {
        let xs = vec![0.0, 1.0, 2.0];
        let ys = vec![0.0, 1.0, 4.0];
        let result = quick_plot(&xs, &ys, None, None, None, 60, 15);
        assert!(result.contains('┌'));
        // Bottom-right corner may be replaced by combined tick+corner char
        assert!(result.contains('┘') || result.contains('┤'));
    }

    #[test]
    fn constant_data() {
        let xs = vec![0.0, 1.0, 2.0, 3.0];
        let ys = vec![5.0, 5.0, 5.0, 5.0];
        let result = quick_plot(&xs, &ys, Some("Constant"), None, None, 60, 15);
        assert!(!result.is_empty());
        assert!(result.contains("Constant"));
    }

    #[test]
    fn single_point() {
        let xs = vec![1.0];
        let ys = vec![1.0];
        let result = quick_plot(&xs, &ys, None, None, None, 40, 12);
        assert!(!result.is_empty());
    }

    #[test]
    fn empty_data() {
        let xs: Vec<f64> = vec![];
        let ys: Vec<f64> = vec![];
        let result = quick_plot(&xs, &ys, None, None, None, 40, 12);
        assert!(!result.is_empty());
    }

    #[test]
    fn negative_range() {
        let xs: Vec<f64> = (-20..=0).map(|i| i as f64).collect();
        let ys: Vec<f64> = xs.iter().map(|x| -x).collect();
        let result = quick_plot(&xs, &ys, Some("Negative"), None, None, 60, 15);
        assert!(result.contains("Negative"));
    }

    #[test]
    fn multi_series() {
        let xs: Vec<f64> = (0..=10).map(|i| i as f64).collect();
        let ys1: Vec<f64> = xs.iter().map(|x| x * x).collect();
        let ys2: Vec<f64> = xs.iter().map(|x| x * 2.0).collect();
        let result = quick_plot_multi(
            &[(&xs, &ys1), (&xs, &ys2)],
            Some("Multi"),
            None,
            None,
            60,
            15,
        );
        assert!(result.contains("Multi"));
    }
}
