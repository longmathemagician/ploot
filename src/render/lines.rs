use crate::canvas::color::TermColor;
use crate::canvas::{BrailleCanvas, DashPattern, SOLID};
use crate::transform::clip_line;

/// Draw a line series on the canvas.
pub fn draw_lines(
    canvas: &mut BrailleCanvas,
    px: &[f64],
    py: &[f64],
    color: TermColor,
    dash: &DashPattern,
) {
    let pw = canvas.pixel_width() as f64;
    let ph = canvas.pixel_height() as f64;
    let len = px.len().min(py.len());
    let mut dash_offset = 0.0;

    for i in 1..len {
        let x0 = px[i - 1];
        let y0 = py[i - 1];
        let x1 = px[i];
        let y1 = py[i];

        if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
            dash_offset = 0.0;
            continue;
        }

        if let Some((cx0, cy0, cx1, cy1)) = clip_line(x0, y0, x1, y1, 0.0, 0.0, pw - 1.0, ph - 1.0)
        {
            canvas.draw_line(
                cx0.round() as i32,
                cy0.round() as i32,
                cx1.round() as i32,
                cy1.round() as i32,
                color,
                dash,
                &mut dash_offset,
            );
        }
    }
}

/// Draw a solid line series (convenience).
pub fn draw_lines_solid(canvas: &mut BrailleCanvas, px: &[f64], py: &[f64], color: TermColor) {
    draw_lines(canvas, px, py, color, &SOLID);
}
