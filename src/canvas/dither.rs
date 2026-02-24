use super::braille::BrailleCanvas;
use super::color::TermColor;

/// Standard 8×8 Bayer ordered dither matrix (values 0–63).
const BAYER_8X8: [[u8; 8]; 8] = [
    [ 0, 32,  8, 40,  2, 34, 10, 42],
    [48, 16, 56, 24, 50, 18, 58, 26],
    [12, 44,  4, 36, 14, 46,  6, 38],
    [60, 28, 52, 20, 62, 30, 54, 22],
    [ 3, 35, 11, 43,  1, 33,  9, 41],
    [51, 19, 59, 27, 49, 17, 57, 25],
    [15, 47,  7, 39, 13, 45,  5, 37],
    [63, 31, 55, 23, 61, 29, 53, 21],
];

/// Returns a dither threshold in (0, 1) for the given global sub-pixel position.
///
/// The `+0.5` centers thresholds so that density 0.0 fills nothing and density 1.0
/// fills everything.
fn bayer_threshold(global_x: usize, global_y: usize) -> f64 {
    (BAYER_8X8[global_y % 8][global_x % 8] as f64 + 0.5) / 64.0
}

/// Fill sub-pixels in a character cell using ordered dithering.
///
/// Iterates all 8 sub-pixels `(col, row)` in the 2×4 Braille cell at
/// `(char_x, char_y)`, computing a global position for each and filling
/// it if `density > bayer_threshold(gx, gy)`.
pub fn fill_cell_dithered(
    canvas: &mut BrailleCanvas,
    char_x: usize,
    char_y: usize,
    density: f64,
    color: TermColor,
) {
    for row in 0..4 {
        for col in 0..2 {
            let gx = char_x * 2 + col;
            let gy = char_y * 4 + row;
            if density > bayer_threshold(gx, gy) {
                canvas.set_pixel(gx, gy, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn bayer_matrix_contains_all_values() {
        let mut values = HashSet::new();
        for row in &BAYER_8X8 {
            for &v in row {
                values.insert(v);
            }
        }
        assert_eq!(values.len(), 64);
        for v in 0..64u8 {
            assert!(values.contains(&v), "missing value {v}");
        }
    }

    #[test]
    fn zero_density_fills_nothing() {
        let mut canvas = BrailleCanvas::new(1, 1);
        fill_cell_dithered(&mut canvas, 0, 0, 0.0, TermColor::White);
        assert_eq!(canvas.cell_byte(0), 0);
    }

    #[test]
    fn full_density_fills_all() {
        let mut canvas = BrailleCanvas::new(1, 1);
        fill_cell_dithered(&mut canvas, 0, 0, 1.0, TermColor::White);
        assert_eq!(canvas.cell_byte(0).count_ones(), 8);
    }

    #[test]
    fn different_cells_different_patterns() {
        let density = 0.4;
        let mut c1 = BrailleCanvas::new(2, 1);
        fill_cell_dithered(&mut c1, 0, 0, density, TermColor::White);
        let b1 = c1.cell_byte(0);

        let mut c2 = BrailleCanvas::new(2, 1);
        fill_cell_dithered(&mut c2, 1, 0, density, TermColor::White);
        let b2 = c2.cell_byte(1);

        assert_ne!(b1, b2, "same density at different positions should produce different patterns");
    }
}
