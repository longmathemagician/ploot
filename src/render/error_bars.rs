use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;

/// Draw vertical error bars (y direction) with caps.
pub fn draw_y_error_bars(
    canvas: &mut BrailleCanvas,
    px: &[f64],
    py_low: &[f64],
    py_high: &[f64],
    color: TermColor,
    cap_width: i32,
) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;
    let len = px.len().min(py_low.len()).min(py_high.len());

    for i in 0..len {
        let cx = px[i];
        let cy_low = py_low[i];
        let cy_high = py_high[i];

        if !cx.is_finite() || !cy_low.is_finite() || !cy_high.is_finite() {
            continue;
        }

        let ix = cx.round() as i32;
        let iy_top = cy_high.min(cy_low).round() as i32;
        let iy_bottom = cy_high.max(cy_low).round() as i32;

        if ix < 0 || ix >= pw {
            continue;
        }

        // Vertical bar
        for y in iy_top.max(0)..=iy_bottom.min(ph - 1) {
            canvas.set_pixel(ix as usize, y as usize, color);
        }

        // Top cap
        let half_cap = cap_width / 2;
        if iy_top >= 0 && iy_top < ph {
            for dx in -half_cap..=half_cap {
                let x = ix + dx;
                if x >= 0 && x < pw {
                    canvas.set_pixel(x as usize, iy_top as usize, color);
                }
            }
        }

        // Bottom cap
        if iy_bottom >= 0 && iy_bottom < ph {
            for dx in -half_cap..=half_cap {
                let x = ix + dx;
                if x >= 0 && x < pw {
                    canvas.set_pixel(x as usize, iy_bottom as usize, color);
                }
            }
        }
    }
}

/// Draw horizontal error bars (x direction) with caps.
pub fn draw_x_error_bars(
    canvas: &mut BrailleCanvas,
    py: &[f64],
    px_low: &[f64],
    px_high: &[f64],
    color: TermColor,
    cap_width: i32,
) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;
    let len = py.len().min(px_low.len()).min(px_high.len());

    for i in 0..len {
        let cy = py[i];
        let cx_low = px_low[i];
        let cx_high = px_high[i];

        if !cy.is_finite() || !cx_low.is_finite() || !cx_high.is_finite() {
            continue;
        }

        let iy = cy.round() as i32;
        let ix_left = cx_low.min(cx_high).round() as i32;
        let ix_right = cx_low.max(cx_high).round() as i32;

        if iy < 0 || iy >= ph {
            continue;
        }

        // Horizontal bar
        for x in ix_left.max(0)..=ix_right.min(pw - 1) {
            canvas.set_pixel(x as usize, iy as usize, color);
        }

        // Left cap
        let half_cap = cap_width / 2;
        if ix_left >= 0 && ix_left < pw {
            for dy in -half_cap..=half_cap {
                let y = iy + dy;
                if y >= 0 && y < ph {
                    canvas.set_pixel(ix_left as usize, y as usize, color);
                }
            }
        }

        // Right cap
        if ix_right >= 0 && ix_right < pw {
            for dy in -half_cap..=half_cap {
                let y = iy + dy;
                if y >= 0 && y < ph {
                    canvas.set_pixel(ix_right as usize, y as usize, color);
                }
            }
        }
    }
}
