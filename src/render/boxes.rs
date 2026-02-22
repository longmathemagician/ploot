use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;

/// Draw a bar chart series. Each bar extends from the baseline to the y value.
pub fn draw_boxes(
    canvas: &mut BrailleCanvas,
    px: &[f64],
    py: &[f64],
    baseline_py: f64,
    color: TermColor,
    box_width_px: f64,
) {
    let pw = canvas.pixel_width() as f64;
    let ph = canvas.pixel_height() as f64;
    let len = px.len().min(py.len());

    for i in 0..len {
        let cx = px[i];
        let cy = py[i];

        if !cx.is_finite() || !cy.is_finite() {
            continue;
        }

        let half_w = (box_width_px / 2.0).max(1.0);
        let x_left = (cx - half_w).round().max(0.0) as usize;
        let x_right = (cx + half_w).round().min(pw - 1.0) as usize;

        let y_top = cy.min(baseline_py).round().max(0.0) as usize;
        let y_bottom = cy.max(baseline_py).round().min(ph - 1.0) as usize;

        // Fill box
        for y in y_top..=y_bottom {
            for x in x_left..=x_right {
                canvas.set_pixel(x, y, color);
            }
        }
    }
}
