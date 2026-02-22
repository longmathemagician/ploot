use super::color::TermColor;
use super::{DashPattern, PIXEL_MAP, SOLID};

/// Dense Braille canvas using `Vec<u8>` dot patterns and `Vec<TermColor>` per-cell colors.
pub struct BrailleCanvas {
    char_width: usize,
    char_height: usize,
    /// Dot pattern for each character cell (row-major).
    cells: Vec<u8>,
    /// Foreground color for each character cell (additive RGB mixing).
    colors: Vec<TermColor>,
}

impl BrailleCanvas {
    /// Creates a new canvas with the given character dimensions.
    pub fn new(char_width: usize, char_height: usize) -> Self {
        let size = char_width * char_height;
        Self {
            char_width,
            char_height,
            cells: vec![0; size],
            colors: vec![TermColor::Default; size],
        }
    }

    /// Pixel width (2 sub-pixels per character column).
    pub fn pixel_width(&self) -> usize {
        self.char_width * 2
    }

    /// Pixel height (4 sub-pixels per character row).
    pub fn pixel_height(&self) -> usize {
        self.char_height * 4
    }

    /// Character width of the canvas.
    pub fn char_width(&self) -> usize {
        self.char_width
    }

    /// Character height of the canvas.
    pub fn char_height(&self) -> usize {
        self.char_height
    }

    /// Returns the dot pattern byte for a cell at the given flat index.
    pub fn cell_byte(&self, idx: usize) -> u8 {
        self.cells[idx]
    }

    /// Returns the color for a cell at the given flat index.
    pub fn cell_color(&self, idx: usize) -> TermColor {
        self.colors[idx]
    }

    /// Sets a single sub-pixel. Out-of-bounds coordinates are silently ignored.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: TermColor) {
        if x >= self.pixel_width() || y >= self.pixel_height() {
            return;
        }
        let col = x / 2;
        let row = y / 4;
        let idx = row * self.char_width + col;
        self.cells[idx] |= PIXEL_MAP[y % 4][x % 2];
        self.colors[idx] = self.colors[idx].mix(color);
    }

    /// Draws a line using Bresenham's algorithm with optional dash pattern.
    pub fn draw_line(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color: TermColor,
        dash: &DashPattern,
    ) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;
        let mut step = 0usize;

        loop {
            if dash.is_on(step) && x >= 0 && y >= 0 {
                self.set_pixel(x as usize, y as usize, color);
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
            step += 1;
        }
    }

    /// Draws a solid line (convenience wrapper).
    pub fn draw_line_solid(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: TermColor) {
        self.draw_line(x0, y0, x1, y1, color, &SOLID);
    }

    /// Clears all cells to blank.
    pub fn clear(&mut self) {
        self.cells.fill(0);
        self.colors.fill(TermColor::Default);
    }

    /// Renders the canvas to a string with ANSI color codes.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for row in 0..self.char_height {
            let mut current_color = TermColor::Default;
            for col in 0..self.char_width {
                let idx = row * self.char_width + col;
                let byte = self.cells[idx];
                let color = self.colors[idx];

                if color != current_color {
                    if current_color != TermColor::Default {
                        out.push_str(TermColor::ansi_reset());
                    }
                    if color != TermColor::Default {
                        out.push_str(color.ansi_fg());
                    }
                    current_color = color;
                }
                out.push(char::from_u32(0x2800 + byte as u32).unwrap());
            }
            if current_color != TermColor::Default {
                out.push_str(TermColor::ansi_reset());
                current_color = TermColor::Default;
            }
            let _ = current_color; // suppress unused warning
            if row < self.char_height - 1 {
                out.push('\n');
            }
        }
        out
    }

    /// Renders without color codes (plain Braille characters).
    pub fn render_plain(&self) -> String {
        let mut out = String::new();
        for row in 0..self.char_height {
            for col in 0..self.char_width {
                let idx = row * self.char_width + col;
                let byte = self.cells[idx];
                out.push(char::from_u32(0x2800 + byte as u32).unwrap());
            }
            if row < self.char_height - 1 {
                out.push('\n');
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions() {
        let c = BrailleCanvas::new(10, 5);
        assert_eq!(c.pixel_width(), 20);
        assert_eq!(c.pixel_height(), 20);
        assert_eq!(c.char_width(), 10);
        assert_eq!(c.char_height(), 5);
    }

    #[test]
    fn empty_render_is_blank_braille() {
        let c = BrailleCanvas::new(3, 2);
        let s = c.render_plain();
        // All chars should be U+2800 (blank braille)
        for ch in s.chars() {
            if ch != '\n' {
                assert_eq!(ch, '\u{2800}');
            }
        }
    }

    #[test]
    fn single_pixel() {
        let mut c = BrailleCanvas::new(2, 2);
        c.set_pixel(0, 0, TermColor::Default);
        let s = c.render_plain();
        // Top-left cell should have dot 1 (0x01) → U+2801
        assert!(s.starts_with('\u{2801}'));
    }

    #[test]
    fn pixel_compositing_bitwise_or() {
        let mut c = BrailleCanvas::new(1, 1);
        // Set two different dots in the same cell
        c.set_pixel(0, 0, TermColor::Default); // bit 0x01
        c.set_pixel(1, 0, TermColor::Default); // bit 0x08
        let s = c.render_plain();
        let ch = s.chars().next().unwrap();
        assert_eq!(ch, char::from_u32(0x2800 + 0x01 + 0x08).unwrap());
    }

    #[test]
    fn color_additive_mixing() {
        let mut c = BrailleCanvas::new(1, 1);
        c.set_pixel(0, 0, TermColor::Blue);
        c.set_pixel(1, 0, TermColor::Red);
        // Blue + Red should mix to Magenta
        let s = c.render();
        assert!(s.contains(TermColor::Magenta.ansi_fg()));
    }

    #[test]
    fn out_of_bounds_ignored() {
        let mut c = BrailleCanvas::new(2, 2);
        c.set_pixel(100, 100, TermColor::Default);
        // Should not panic, canvas should remain blank
        let s = c.render_plain();
        for ch in s.chars() {
            if ch != '\n' {
                assert_eq!(ch, '\u{2800}');
            }
        }
    }

    #[test]
    fn horizontal_line() {
        let mut c = BrailleCanvas::new(5, 1);
        c.draw_line_solid(0, 0, 9, 0, TermColor::Default);
        let s = c.render_plain();
        // Every character cell in the row should have at least one dot set
        for ch in s.chars() {
            assert_ne!(ch, '\u{2800}');
        }
    }

    #[test]
    fn vertical_line() {
        let mut c = BrailleCanvas::new(1, 3);
        c.draw_line_solid(0, 0, 0, 11, TermColor::Default);
        let s = c.render_plain();
        for ch in s.chars() {
            if ch != '\n' {
                assert_ne!(ch, '\u{2800}');
            }
        }
    }

    #[test]
    fn diagonal_line() {
        let mut c = BrailleCanvas::new(5, 5);
        c.draw_line_solid(0, 0, 9, 19, TermColor::Default);
        let s = c.render_plain();
        let non_blank: usize = s
            .chars()
            .filter(|&ch| ch != '\n' && ch != '\u{2800}')
            .count();
        assert!(non_blank > 0);
    }

    #[test]
    fn clear_resets() {
        let mut c = BrailleCanvas::new(3, 3);
        c.set_pixel(0, 0, TermColor::Blue);
        c.clear();
        let s = c.render_plain();
        for ch in s.chars() {
            if ch != '\n' {
                assert_eq!(ch, '\u{2800}');
            }
        }
    }

    #[test]
    fn dash_pattern_line() {
        use super::super::DASH;
        let mut c = BrailleCanvas::new(10, 1);
        c.draw_line(0, 0, 19, 0, TermColor::Default, &DASH);
        let s = c.render_plain();
        // Some cells should have dots, some should be blank (due to dash gaps)
        let has_dot = s.chars().any(|ch| ch != '\n' && ch != '\u{2800}');
        let has_blank = s.chars().any(|ch| ch == '\u{2800}');
        assert!(has_dot);
        assert!(has_blank);
    }
}
