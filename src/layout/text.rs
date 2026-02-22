use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;

/// Character grid for compositing text, borders, and Braille canvas output.
pub struct TextGrid {
    width: usize,
    height: usize,
    chars: Vec<char>,
    colors: Vec<TermColor>,
}

impl TextGrid {
    /// Creates a new grid filled with spaces.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chars: vec![' '; width * height],
            colors: vec![TermColor::Default; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Sets a character at the given position.
    pub fn put_char(&mut self, col: usize, row: usize, ch: char, color: TermColor) {
        if col < self.width && row < self.height {
            let idx = row * self.width + col;
            self.chars[idx] = ch;
            self.colors[idx] = color;
        }
    }

    /// Writes a string starting at `(col, row)`, left-aligned.
    pub fn put_str(&mut self, col: usize, row: usize, s: &str, color: TermColor) {
        for (i, ch) in s.chars().enumerate() {
            self.put_char(col + i, row, ch, color);
        }
    }

    /// Writes a string centered within `[col_start, col_start + width)`.
    pub fn put_str_centered(
        &mut self,
        col_start: usize,
        width: usize,
        row: usize,
        s: &str,
        color: TermColor,
    ) {
        let len = s.chars().count();
        if len >= width {
            self.put_str(col_start, row, s, color);
        } else {
            let offset = (width - len) / 2;
            self.put_str(col_start + offset, row, s, color);
        }
    }

    /// Writes a string right-aligned ending at column `col_end` (exclusive).
    pub fn put_str_right_aligned(&mut self, col_end: usize, row: usize, s: &str, color: TermColor) {
        let len = s.chars().count();
        if len <= col_end {
            self.put_str(col_end - len, row, s, color);
        }
    }

    /// Composites a BrailleCanvas into the grid at the given offset.
    pub fn blit_braille(&mut self, canvas: &BrailleCanvas, col_offset: usize, row_offset: usize) {
        let rendered = canvas.render();
        for (row_idx, line) in rendered.lines().enumerate() {
            let grid_row = row_offset + row_idx;
            if grid_row >= self.height {
                break;
            }
            let mut col = col_offset;
            let mut chars = line.chars().peekable();
            while let Some(&ch) = chars.peek() {
                if ch == '\x1b' {
                    // Skip ANSI escape sequence
                    for c in chars.by_ref() {
                        if c == 'm' {
                            break;
                        }
                    }
                    continue;
                }
                chars.next();
                if col < self.width {
                    // Check if this is a non-blank braille character
                    if ch as u32 >= 0x2800 && ch as u32 <= 0x28FF {
                        self.put_char(col, grid_row, ch, TermColor::Default);
                    }
                }
                col += 1;
            }
        }
        // Instead, blit directly from canvas internal state for accurate colors
        self.blit_braille_direct(canvas, col_offset, row_offset);
    }

    /// Direct blit from canvas data (avoids parsing ANSI from render output).
    fn blit_braille_direct(
        &mut self,
        canvas: &BrailleCanvas,
        col_offset: usize,
        row_offset: usize,
    ) {
        for crow in 0..canvas.char_height() {
            let grid_row = row_offset + crow;
            if grid_row >= self.height {
                break;
            }
            for ccol in 0..canvas.char_width() {
                let grid_col = col_offset + ccol;
                if grid_col >= self.width {
                    break;
                }
                let cidx = crow * canvas.char_width() + ccol;
                let byte = canvas.cell_byte(cidx);
                let color = canvas.cell_color(cidx);
                let ch = char::from_u32(0x2800 + byte as u32).unwrap();
                self.put_char(grid_col, grid_row, ch, color);
            }
        }
    }

    /// Renders the grid to a string with ANSI color codes.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for row in 0..self.height {
            let mut current_color = TermColor::Default;
            for col in 0..self.width {
                let idx = row * self.width + col;
                let ch = self.chars[idx];
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
                out.push(ch);
            }
            if current_color != TermColor::Default {
                out.push_str(TermColor::ansi_reset());
            }
            if row < self.height - 1 {
                out.push('\n');
            }
        }
        out
    }

    /// Renders without color codes (for testing).
    pub fn render_plain(&self) -> String {
        let mut out = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                out.push(self.chars[idx]);
            }
            if row < self.height - 1 {
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
    fn new_grid_is_spaces() {
        let g = TextGrid::new(10, 5);
        let s = g.render_plain();
        assert!(s.chars().all(|c| c == ' ' || c == '\n'));
    }

    #[test]
    fn put_str_works() {
        let mut g = TextGrid::new(20, 3);
        g.put_str(0, 0, "Hello", TermColor::Default);
        let s = g.render_plain();
        assert!(s.starts_with("Hello"));
    }

    #[test]
    fn centered_text() {
        let mut g = TextGrid::new(20, 1);
        g.put_str_centered(0, 20, 0, "Hi", TermColor::Default);
        let s = g.render_plain();
        let trimmed = s.trim();
        assert_eq!(trimmed, "Hi");
        // Should be roughly centered
        let leading = s.len() - s.trim_start().len();
        assert!(leading >= 8);
    }

    #[test]
    fn right_aligned() {
        let mut g = TextGrid::new(10, 1);
        g.put_str_right_aligned(10, 0, "Hi", TermColor::Default);
        let s = g.render_plain();
        assert!(s.ends_with("Hi"));
    }
}
