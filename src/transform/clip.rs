/// Cohen-Sutherland outcode bits.
const INSIDE: u8 = 0b0000;
const LEFT: u8 = 0b0001;
const RIGHT: u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const TOP: u8 = 0b1000;

fn outcode(x: f64, y: f64, x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> u8 {
    let mut code = INSIDE;
    if x < x_min {
        code |= LEFT;
    } else if x > x_max {
        code |= RIGHT;
    }
    if y < y_min {
        code |= BOTTOM;
    } else if y > y_max {
        code |= TOP;
    }
    code
}

#[allow(clippy::too_many_arguments)]
/// Clips a line segment to the rectangle `[x_min, x_max] × [y_min, y_max]`.
///
/// Returns `Some((x0, y0, x1, y1))` with the clipped segment, or `None` if
/// the line is entirely outside.
pub fn clip_line(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
) -> Option<(f64, f64, f64, f64)> {
    let mut code0 = outcode(x0, y0, x_min, y_min, x_max, y_max);
    let mut code1 = outcode(x1, y1, x_min, y_min, x_max, y_max);

    loop {
        if (code0 | code1) == INSIDE {
            // Both inside
            return Some((x0, y0, x1, y1));
        }
        if (code0 & code1) != INSIDE {
            // Both in the same outside zone
            return None;
        }

        // Pick the point that is outside
        let code_out = if code0 != INSIDE { code0 } else { code1 };

        let (x, y);
        if code_out & TOP != 0 {
            x = x0 + (x1 - x0) * (y_max - y0) / (y1 - y0);
            y = y_max;
        } else if code_out & BOTTOM != 0 {
            x = x0 + (x1 - x0) * (y_min - y0) / (y1 - y0);
            y = y_min;
        } else if code_out & RIGHT != 0 {
            y = y0 + (y1 - y0) * (x_max - x0) / (x1 - x0);
            x = x_max;
        } else {
            // LEFT
            y = y0 + (y1 - y0) * (x_min - x0) / (x1 - x0);
            x = x_min;
        }

        if code_out == code0 {
            x0 = x;
            y0 = y;
            code0 = outcode(x0, y0, x_min, y_min, x_max, y_max);
        } else {
            x1 = x;
            y1 = y;
            code1 = outcode(x1, y1, x_min, y_min, x_max, y_max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_inside() {
        let r = clip_line(1.0, 1.0, 5.0, 5.0, 0.0, 0.0, 10.0, 10.0);
        assert_eq!(r, Some((1.0, 1.0, 5.0, 5.0)));
    }

    #[test]
    fn fully_outside_same_side() {
        let r = clip_line(-5.0, -5.0, -1.0, -1.0, 0.0, 0.0, 10.0, 10.0);
        assert_eq!(r, None);
    }

    #[test]
    fn partial_clip() {
        let r = clip_line(-5.0, 5.0, 5.0, 5.0, 0.0, 0.0, 10.0, 10.0);
        let (x0, y0, x1, y1) = r.unwrap();
        assert!((x0 - 0.0).abs() < 1e-9);
        assert!((y0 - 5.0).abs() < 1e-9);
        assert!((x1 - 5.0).abs() < 1e-9);
        assert!((y1 - 5.0).abs() < 1e-9);
    }

    #[test]
    fn diagonal_clip() {
        let r = clip_line(-1.0, -1.0, 11.0, 11.0, 0.0, 0.0, 10.0, 10.0);
        let (x0, y0, x1, y1) = r.unwrap();
        assert!((x0 - 0.0).abs() < 1e-9);
        assert!((y0 - 0.0).abs() < 1e-9);
        assert!((x1 - 10.0).abs() < 1e-9);
        assert!((y1 - 10.0).abs() < 1e-9);
    }

    #[test]
    fn fully_outside_opposite_sides_no_intersection() {
        // Line from far left-bottom to far left-top, entirely left of viewport
        let r = clip_line(-5.0, -5.0, -5.0, 15.0, 0.0, 0.0, 10.0, 10.0);
        assert_eq!(r, None);
    }
}
