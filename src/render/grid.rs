use crate::canvas::BrailleCanvas;
use crate::canvas::DashPattern;
use crate::canvas::color::TermColor;
use crate::layout::TickSet;
use crate::transform::CoordinateMapper;

/// Draw grid lines on the canvas at tick positions.
///
/// Grid lines are drawn BEFORE data series so data renders on top.
/// The mapper is expected to produce cell-aligned pixels for tick values,
/// so no post-hoc snapping is needed.
#[allow(clippy::too_many_arguments)]
pub fn draw_grid(
    canvas: &mut BrailleCanvas,
    mapper: &CoordinateMapper,
    x_ticks: &TickSet,
    y_ticks: &TickSet,
    x_grid: bool,
    y_grid: bool,
    x_grid_color: TermColor,
    y_grid_color: TermColor,
    x_grid_dash: &DashPattern,
    y_grid_dash: &DashPattern,
) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;

    // Vertical grid lines at x tick positions
    if x_grid {
        for &(val, _) in &x_ticks.ticks {
            let (px, _) = mapper.data_to_pixel(val, 0.0);
            let ix = px.round() as i32;
            if ix >= 0 && ix < pw {
                for y in 0..ph {
                    if x_grid_dash.is_on(y as usize) {
                        canvas.set_pixel(ix as usize, y as usize, x_grid_color);
                    }
                }
            }
        }
    }

    // Horizontal grid lines at y tick positions
    if y_grid {
        for &(val, _) in &y_ticks.ticks {
            let (_, py) = mapper.data_to_pixel(0.0, val);
            let iy = py.round() as i32;
            if iy >= 0 && iy < ph {
                for x in 0..pw {
                    if y_grid_dash.is_on(x as usize) {
                        canvas.set_pixel(x as usize, iy as usize, y_grid_color);
                    }
                }
            }
        }
    }
}

/// Draw minor grid lines between major tick positions.
///
/// Minor grid lines are midpoints between major ticks. They intentionally
/// land at non-cell-aligned sub-pixel positions, which makes them visually
/// distinct from major grid lines.
#[allow(clippy::too_many_arguments)]
pub fn draw_minor_grid(
    canvas: &mut BrailleCanvas,
    mapper: &CoordinateMapper,
    x_ticks: &TickSet,
    y_ticks: &TickSet,
    x_minor: bool,
    y_minor: bool,
    x_color: TermColor,
    y_color: TermColor,
    x_dash: &DashPattern,
    y_dash: &DashPattern,
) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;

    // Minor vertical grid lines (midpoints between major x ticks)
    if x_minor && x_ticks.ticks.len() >= 2 {
        for w in x_ticks.ticks.windows(2) {
            let mid = (w[0].0 + w[1].0) / 2.0;
            let (px, _) = mapper.data_to_pixel(mid, 0.0);
            let ix = px.round() as i32;
            if ix >= 0 && ix < pw {
                for y in 0..ph {
                    if x_dash.is_on(y as usize) {
                        canvas.set_pixel(ix as usize, y as usize, x_color);
                    }
                }
            }
        }
    }

    // Minor horizontal grid lines (midpoints between major y ticks)
    if y_minor && y_ticks.ticks.len() >= 2 {
        for w in y_ticks.ticks.windows(2) {
            let mid = (w[0].0 + w[1].0) / 2.0;
            let (_, py) = mapper.data_to_pixel(0.0, mid);
            let iy = py.round() as i32;
            if iy >= 0 && iy < ph {
                for x in 0..pw {
                    if y_dash.is_on(x as usize) {
                        canvas.set_pixel(x as usize, iy as usize, y_color);
                    }
                }
            }
        }
    }
}
