use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;

/// Fill the area between two y curves for each pixel column.
pub fn draw_fill_between(
    canvas: &mut BrailleCanvas,
    px: &[f64],
    py1: &[f64],
    py2: &[f64],
    color: TermColor,
) {
    let pw = canvas.pixel_width() as f64;
    let ph = canvas.pixel_height() as f64;
    let len = px.len().min(py1.len()).min(py2.len());

    for i in 0..len {
        let cx = px[i];
        let cy1 = py1[i];
        let cy2 = py2[i];

        if !cx.is_finite() || !cy1.is_finite() || !cy2.is_finite() {
            continue;
        }

        let ix = cx.round() as i32;
        if ix < 0 || ix >= pw as i32 {
            continue;
        }

        let y_top = cy1.min(cy2).round().max(0.0) as usize;
        let y_bottom = cy1.max(cy2).round().min(ph - 1.0) as usize;

        for y in y_top..=y_bottom {
            canvas.set_pixel(ix as usize, y, color);
        }
    }
}
