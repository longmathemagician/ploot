use crate::api::options::PointSymbol;
use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;

/// Draw point markers at each data position.
pub fn draw_points(
    canvas: &mut BrailleCanvas,
    px: &[f64],
    py: &[f64],
    color: TermColor,
    symbol: PointSymbol,
) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;
    let len = px.len().min(py.len());

    for i in 0..len {
        let x = px[i];
        let y = py[i];

        if !x.is_finite() || !y.is_finite() {
            continue;
        }

        let ix = x.round() as i32;
        let iy = y.round() as i32;

        if ix < 0 || iy < 0 || ix >= pw || iy >= ph {
            continue;
        }

        draw_marker(canvas, ix, iy, color, symbol);
    }
}

fn draw_marker(canvas: &mut BrailleCanvas, x: i32, y: i32, color: TermColor, symbol: PointSymbol) {
    let pw = canvas.pixel_width() as i32;
    let ph = canvas.pixel_height() as i32;

    match symbol {
        PointSymbol::Dot => {
            set_if_valid(canvas, x, y, pw, ph, color);
        }
        PointSymbol::Cross => {
            set_if_valid(canvas, x, y, pw, ph, color);
            set_if_valid(canvas, x - 1, y, pw, ph, color);
            set_if_valid(canvas, x + 1, y, pw, ph, color);
            set_if_valid(canvas, x, y - 1, pw, ph, color);
            set_if_valid(canvas, x, y + 1, pw, ph, color);
        }
        PointSymbol::Circle => {
            // Small ring: 8 surrounding pixels
            for &(dx, dy) in &[
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                set_if_valid(canvas, x + dx, y + dy, pw, ph, color);
            }
        }
        PointSymbol::Diamond => {
            set_if_valid(canvas, x, y - 1, pw, ph, color);
            set_if_valid(canvas, x - 1, y, pw, ph, color);
            set_if_valid(canvas, x + 1, y, pw, ph, color);
            set_if_valid(canvas, x, y + 1, pw, ph, color);
        }
        PointSymbol::Triangle => {
            set_if_valid(canvas, x, y - 1, pw, ph, color);
            set_if_valid(canvas, x - 1, y + 1, pw, ph, color);
            set_if_valid(canvas, x, y + 1, pw, ph, color);
            set_if_valid(canvas, x + 1, y + 1, pw, ph, color);
        }
        PointSymbol::Square => {
            for &(dx, dy) in &[
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                set_if_valid(canvas, x + dx, y + dy, pw, ph, color);
            }
        }
    }
}

fn set_if_valid(canvas: &mut BrailleCanvas, x: i32, y: i32, pw: i32, ph: i32, color: TermColor) {
    if x >= 0 && y >= 0 && x < pw && y < ph {
        canvas.set_pixel(x as usize, y as usize, color);
    }
}
