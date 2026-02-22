use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;

#[allow(clippy::too_many_arguments)]
/// Draw box-and-whisker plots.
///
/// For each data point, draws:
/// - A box from Q1 to Q3
/// - A horizontal line at the median
/// - Whiskers extending to min and max
/// - Caps at whisker endpoints
pub fn draw_box_and_whisker(
    canvas: &mut BrailleCanvas,
    px: &[f64],
    py_min: &[f64],
    py_q1: &[f64],
    py_median: &[f64],
    py_q3: &[f64],
    py_max: &[f64],
    color: TermColor,
    box_width_px: f64,
) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;
    let len = px
        .len()
        .min(py_min.len())
        .min(py_q1.len())
        .min(py_median.len())
        .min(py_q3.len())
        .min(py_max.len());

    let half_w = (box_width_px / 2.0).max(1.0) as i32;
    let cap_half = (half_w / 2).max(1);

    for i in 0..len {
        let cx = px[i].round() as i32;
        let y_min = py_min[i].round() as i32;
        let y_q1 = py_q1[i].round() as i32;
        let y_med = py_median[i].round() as i32;
        let y_q3 = py_q3[i].round() as i32;
        let y_max = py_max[i].round() as i32;

        if cx < 0 || cx >= pw {
            continue;
        }

        // Note: in pixel space, lower values are higher on screen (y inverted).
        // y_max pixel < y_q3 pixel < y_median pixel < y_q1 pixel < y_min pixel
        // (assuming the mapper inverted y correctly)

        // Whisker: min to Q1
        let whisker_top = y_min.min(y_max).max(0);
        let whisker_bottom = y_min.max(y_max).min(ph - 1);
        for y in whisker_top..=whisker_bottom {
            set_if_valid(canvas, cx, y, pw, ph, color);
        }

        // Box outline: Q1 to Q3
        let box_top = y_q1.min(y_q3).max(0);
        let box_bottom = y_q1.max(y_q3).min(ph - 1);
        for y in box_top..=box_bottom {
            set_if_valid(canvas, cx - half_w, y, pw, ph, color);
            set_if_valid(canvas, cx + half_w, y, pw, ph, color);
        }
        // Top and bottom of box
        for x in (cx - half_w)..=(cx + half_w) {
            set_if_valid(canvas, x, box_top, pw, ph, color);
            set_if_valid(canvas, x, box_bottom, pw, ph, color);
        }

        // Median line
        for x in (cx - half_w)..=(cx + half_w) {
            set_if_valid(canvas, x, y_med, pw, ph, color);
        }

        // Caps at whisker endpoints
        // Note: we use the actual mapped values, not sorted
        for x in (cx - cap_half)..=(cx + cap_half) {
            set_if_valid(canvas, x, y_min.min(y_max), pw, ph, color);
            set_if_valid(canvas, x, y_min.max(y_max), pw, ph, color);
        }
    }
}

fn set_if_valid(canvas: &mut BrailleCanvas, x: i32, y: i32, pw: i32, ph: i32, color: TermColor) {
    if x >= 0 && y >= 0 && x < pw && y < ph {
        canvas.set_pixel(x as usize, y as usize, color);
    }
}
