//! Braille canvas and color primitives.
//!
//! Provides [`BrailleCanvas`] for 2×4 sub-pixel rendering using Unicode Braille
//! characters (U+2800–U+28FF), plus [`TermColor`] and [`DashPattern`] types.

/// [`BrailleCanvas`] implementation.
pub mod braille;
/// Terminal color types and palette.
pub mod color;
/// Colormap for scalar-to-color mapping.
pub mod colormap;
/// Depth-buffered canvas for z-correct rendering.
pub mod depth;
/// Ordered dithering for smooth density gradients.
pub mod dither;

pub use braille::BrailleCanvas;
pub use color::{PALETTE, TermColor};
pub use colormap::{ColorDensity, ColorMapType, map_color};
pub use depth::DepthCanvas;
pub use dither::fill_cell_dithered;

/// Braille bit positions: `PIXEL_MAP[y % 4][x % 2]` gives the bit mask for a sub-pixel.
pub const PIXEL_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08], // row 0
    [0x02, 0x10], // row 1
    [0x04, 0x20], // row 2
    [0x40, 0x80], // row 3
];

/// A dash pattern defined as alternating on/off pixel counts.
#[derive(Debug, Clone, Copy)]
pub struct DashPattern {
    /// Alternating on/off lengths in pixels. First element is "on".
    pub segments: &'static [usize],
}

/// Solid line (every pixel drawn).
pub const SOLID: DashPattern = DashPattern { segments: &[1] };
/// Dashed line: 6 on, 4 off.
pub const DASH: DashPattern = DashPattern { segments: &[6, 4] };
/// Dotted line: 2 on, 4 off.
pub const DOT: DashPattern = DashPattern { segments: &[2, 4] };
/// Dot-dash line: 2 on, 4 off, 6 on, 4 off.
pub const DOT_DASH: DashPattern = DashPattern {
    segments: &[2, 4, 6, 4],
};
/// Dot-dot-dash line: 2 on, 4 off, 2 on, 4 off, 6 on, 4 off.
pub const DOT_DOT_DASH: DashPattern = DashPattern {
    segments: &[2, 4, 2, 4, 6, 4],
};
/// Small dot line: 1 on, 3 off.
pub const SMALL_DOT: DashPattern = DashPattern { segments: &[1, 3] };

impl DashPattern {
    /// Returns whether a pixel at the given step count along a line should be drawn.
    pub fn is_on(&self, step: usize) -> bool {
        if self.segments.len() <= 1 {
            return true; // solid
        }
        let total: usize = self.segments.iter().sum();
        let pos = step % total;
        let mut acc = 0;
        for (i, &len) in self.segments.iter().enumerate() {
            acc += len;
            if pos < acc {
                return i % 2 == 0; // even index = on, odd = off
            }
        }
        true
    }

    /// Returns whether a pixel at the given Euclidean distance along a line should be drawn.
    ///
    /// Same logic as [`is_on`](Self::is_on) but operates on `f64` distance values,
    /// preserving visual dash spacing regardless of line slope.
    pub fn is_on_at_distance(&self, distance: f64) -> bool {
        if self.segments.len() <= 1 {
            return true; // solid
        }
        let total: f64 = self.segments.iter().map(|&s| s as f64).sum();
        let pos = distance % total;
        let mut acc = 0.0;
        for (i, &len) in self.segments.iter().enumerate() {
            acc += len as f64;
            if pos < acc {
                return i % 2 == 0;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_map_covers_all_bits() {
        let mut all = 0u8;
        for row in &PIXEL_MAP {
            for &bit in row {
                all |= bit;
            }
        }
        assert_eq!(all, 0xFF);
    }

    #[test]
    fn solid_always_on() {
        for i in 0..20 {
            assert!(SOLID.is_on(i));
        }
    }

    #[test]
    fn dash_pattern_alternates() {
        // DASH: 6 on, 4 off
        for i in 0..6 {
            assert!(DASH.is_on(i), "step {i} should be on");
        }
        for i in 6..10 {
            assert!(!DASH.is_on(i), "step {i} should be off");
        }
        // wraps around
        assert!(DASH.is_on(10));
    }
}
