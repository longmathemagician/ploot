use super::PIXEL_MAP;
use super::braille::BrailleCanvas;
use super::color::TermColor;
use super::colormap::ColorDensity;

/// Sub-pixel fill order within a 2x4 Braille cell for perceptually uniform density.
///
/// Positions are (column, row) within the cell, ordered to spread dots
/// evenly across the cell as density increases from 1 to 8.
const DENSITY_FILL_ORDER: [(usize, usize); 8] = [
    (0, 0), // top-left
    (1, 3), // bottom-right
    (1, 1), // middle-right
    (0, 2), // middle-left
    (0, 1), // upper-left
    (1, 0), // top-right
    (1, 2), // lower-right
    (0, 3), // bottom-left
];

/// Z-buffer wrapper around BrailleCanvas for depth-correct rendering.
///
/// Since Braille canvas uses additive OR compositing (can't erase pixels),
/// painter's algorithm won't work. The depth buffer enables front-to-back
/// rendering with correct occlusion.
pub struct DepthCanvas {
    canvas: BrailleCanvas,
    /// Per sub-pixel depth, initialized to infinity.
    depth: Vec<f64>,
}

impl DepthCanvas {
    /// Create a new depth canvas with the given character dimensions.
    pub fn new(char_width: usize, char_height: usize) -> Self {
        let pixel_count = char_width * 2 * char_height * 4;
        Self {
            canvas: BrailleCanvas::new(char_width, char_height),
            depth: vec![f64::INFINITY; pixel_count],
        }
    }

    /// Width in sub-pixels.
    pub fn pixel_width(&self) -> usize {
        self.canvas.pixel_width()
    }

    /// Height in sub-pixels.
    pub fn pixel_height(&self) -> usize {
        self.canvas.pixel_height()
    }

    /// Set a sub-pixel only if the given depth is closer than stored.
    pub fn set_pixel_depth(&mut self, x: usize, y: usize, z: f64, color: TermColor) {
        if x >= self.pixel_width() || y >= self.pixel_height() {
            return;
        }
        let idx = y * self.pixel_width() + x;
        if z < self.depth[idx] {
            self.depth[idx] = z;
            self.canvas.set_pixel(x, y, color);
        }
    }

    /// Draw a line with linear depth interpolation and per-pixel z-test.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_line_depth(
        &mut self,
        x0: i32,
        y0: i32,
        z0: f64,
        x1: i32,
        y1: i32,
        z1: f64,
        color: TermColor,
    ) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        let total_steps = dx.max(-dy) as f64;

        loop {
            // Interpolate depth
            let t = if total_steps > 0.0 {
                let steps_done = ((x - x0).abs().max((y - y0).abs())) as f64;
                steps_done / total_steps
            } else {
                0.0
            };
            let z = z0 + t * (z1 - z0);

            if x >= 0 && y >= 0 {
                self.set_pixel_depth(x as usize, y as usize, z, color);
            }

            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Fill N dots in a character cell using the density fill order, with depth check.
    pub fn fill_cell_density(&mut self, char_x: usize, char_y: usize, cd: ColorDensity, z: f64) {
        let base_px = char_x * 2;
        let base_py = char_y * 4;
        let n = (cd.density * 8.0).round().clamp(0.0, 8.0) as usize;

        for &(col, row) in DENSITY_FILL_ORDER.iter().take(n) {
            let px = base_px + col;
            let py = base_py + row;
            self.set_pixel_depth(px, py, z, cd.color);
        }
    }

    /// Fill N dots in a character cell, writing directly without depth check.
    pub fn fill_cell_density_no_depth(&mut self, char_x: usize, char_y: usize, cd: ColorDensity) {
        let base_px = char_x * 2;
        let base_py = char_y * 4;
        let n = (cd.density * 8.0).round().clamp(0.0, 8.0) as usize;
        let _ = PIXEL_MAP; // reference for clarity

        for &(col, row) in DENSITY_FILL_ORDER.iter().take(n) {
            let px = base_px + col;
            let py = base_py + row;
            self.canvas.set_pixel(px, py, cd.color);
        }
    }

    /// Borrow the underlying canvas.
    pub fn canvas(&self) -> &BrailleCanvas {
        &self.canvas
    }

    /// Consume and return the underlying canvas.
    pub fn into_canvas(self) -> BrailleCanvas {
        self.canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closer_overwrites_farther() {
        let mut dc = DepthCanvas::new(2, 2);
        // Set a pixel at depth 10
        dc.set_pixel_depth(0, 0, 10.0, TermColor::Red);
        // Overwrite with closer depth 5
        dc.set_pixel_depth(0, 0, 5.0, TermColor::Blue);
        // The pixel should be set (both colors mixed due to additive)
        let byte = dc.canvas().cell_byte(0);
        assert_ne!(byte, 0);
    }

    #[test]
    fn farther_rejected() {
        let mut dc = DepthCanvas::new(2, 2);
        // Set at depth 5
        dc.set_pixel_depth(0, 0, 5.0, TermColor::Red);
        // Try to set at depth 10 - should not change depth
        dc.set_pixel_depth(0, 0, 10.0, TermColor::Blue);
        // Depth should still be 5
        assert!((dc.depth[0] - 5.0).abs() < 1e-9);
    }

    #[test]
    fn fill_cell_density_fills_correct_count() {
        let mut dc = DepthCanvas::new(1, 1);
        let cd = ColorDensity {
            color: TermColor::Green,
            density: 0.5,
        };
        dc.fill_cell_density(0, 0, cd, 1.0);
        let byte = dc.canvas().cell_byte(0);
        assert_eq!(byte.count_ones(), 4);
    }

    #[test]
    fn line_depth_basic() {
        let mut dc = DepthCanvas::new(5, 1);
        dc.draw_line_depth(0, 0, 1.0, 9, 0, 1.0, TermColor::White);
        // Should have some pixels set
        let s = dc.canvas().render_plain();
        let non_blank = s.chars().filter(|&c| c != '\u{2800}' && c != '\n').count();
        assert!(non_blank > 0);
    }

    #[test]
    fn density_fill_order_covers_all() {
        let mut cols = [false; 2];
        let mut rows = [false; 4];
        let mut positions = std::collections::HashSet::new();
        for &(c, r) in &DENSITY_FILL_ORDER {
            assert!(c < 2);
            assert!(r < 4);
            cols[c] = true;
            rows[r] = true;
            positions.insert((c, r));
        }
        assert_eq!(positions.len(), 8);
        assert!(cols.iter().all(|&v| v));
        assert!(rows.iter().all(|&v| v));
    }
}
