use super::color::TermColor;

/// Colormap type for mapping scalar values to colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMapType {
    /// Blue -> Cyan -> Green -> Yellow -> Red -> Magenta -> White (heat ramp).
    Heat,
    /// Density-only using White color (grayscale-like).
    Gray,
    /// Full rainbow: Blue -> Cyan -> Green -> Yellow -> Red -> Magenta.
    Rainbow,
    /// Blue (low) -> White (mid) -> Red (high) diverging map.
    BlueRed,
}

/// A color + density pair for Braille rendering.
#[derive(Debug, Clone, Copy)]
pub struct ColorDensity {
    /// ANSI terminal color.
    pub color: TermColor,
    /// Number of Braille dots to fill in a cell (0-8).
    pub density: u8,
}

/// Sub-pixel fill order within a 2x4 Braille cell for perceptually uniform density.
///
/// Positions are (column, row) within the cell, ordered to spread dots
/// evenly across the cell as density increases from 1 to 8.
pub const DENSITY_FILL_ORDER: [(usize, usize); 8] = [
    (0, 0), // top-left
    (1, 3), // bottom-right
    (1, 1), // middle-right
    (0, 2), // middle-left
    (0, 1), // upper-left
    (1, 0), // top-right
    (1, 2), // lower-right
    (0, 3), // bottom-left
];

/// The heat colormap band definitions: (color, start_t, end_t).
/// 7 colors with 8 density levels each = 56 perceptual levels.
const HEAT_BANDS: [(TermColor, f64, f64); 7] = [
    (TermColor::Blue, 0.0, 1.0 / 7.0),
    (TermColor::Cyan, 1.0 / 7.0, 2.0 / 7.0),
    (TermColor::Green, 2.0 / 7.0, 3.0 / 7.0),
    (TermColor::Yellow, 3.0 / 7.0, 4.0 / 7.0),
    (TermColor::Red, 4.0 / 7.0, 5.0 / 7.0),
    (TermColor::Magenta, 5.0 / 7.0, 6.0 / 7.0),
    (TermColor::White, 6.0 / 7.0, 1.0),
];

const RAINBOW_BANDS: [(TermColor, f64, f64); 6] = [
    (TermColor::Blue, 0.0, 1.0 / 6.0),
    (TermColor::Cyan, 1.0 / 6.0, 2.0 / 6.0),
    (TermColor::Green, 2.0 / 6.0, 3.0 / 6.0),
    (TermColor::Yellow, 3.0 / 6.0, 4.0 / 6.0),
    (TermColor::Red, 4.0 / 6.0, 5.0 / 6.0),
    (TermColor::Magenta, 5.0 / 6.0, 1.0),
];

/// Maps a normalized value `t` in [0, 1] to a `ColorDensity`.
pub fn map_color(t: f64, cmap: ColorMapType) -> ColorDensity {
    let t = t.clamp(0.0, 1.0);

    match cmap {
        ColorMapType::Heat => map_banded(t, &HEAT_BANDS),
        ColorMapType::Gray => {
            let density = (t * 8.0).round() as u8;
            ColorDensity {
                color: TermColor::White,
                density: density.min(8),
            }
        }
        ColorMapType::Rainbow => map_banded(t, &RAINBOW_BANDS),
        ColorMapType::BlueRed => {
            if t < 0.5 {
                // Blue range: density decreases from 8 to 1 as t goes 0 -> 0.5
                let local_t = 1.0 - t / 0.5;
                let density = (local_t * 7.0 + 1.0).round() as u8;
                ColorDensity {
                    color: TermColor::Blue,
                    density: density.clamp(1, 8),
                }
            } else {
                // Red range: density increases from 1 to 8 as t goes 0.5 -> 1.0
                let local_t = (t - 0.5) / 0.5;
                let density = (local_t * 7.0 + 1.0).round() as u8;
                ColorDensity {
                    color: TermColor::Red,
                    density: density.clamp(1, 8),
                }
            }
        }
    }
}

fn map_banded(t: f64, bands: &[(TermColor, f64, f64)]) -> ColorDensity {
    for &(color, start, end) in bands {
        if t >= start && t <= end + f64::EPSILON {
            let band_width = end - start;
            let local_t = if band_width.abs() < f64::EPSILON {
                0.5
            } else {
                (t - start) / band_width
            };
            let density = (local_t * 7.0 + 1.0).round() as u8;
            return ColorDensity {
                color,
                density: density.clamp(1, 8),
            };
        }
    }
    // Fallback for t == 1.0
    let (color, _, _) = bands[bands.len() - 1];
    ColorDensity { color, density: 8 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_map_endpoints() {
        let low = map_color(0.0, ColorMapType::Heat);
        assert_eq!(low.color, TermColor::Blue);
        assert!(low.density >= 1);

        let high = map_color(1.0, ColorMapType::Heat);
        assert_eq!(high.color, TermColor::White);
        assert_eq!(high.density, 8);
    }

    #[test]
    fn gray_map_range() {
        let low = map_color(0.0, ColorMapType::Gray);
        assert_eq!(low.density, 0);

        let high = map_color(1.0, ColorMapType::Gray);
        assert_eq!(high.density, 8);
    }

    #[test]
    fn all_maps_valid_density() {
        for cmap in [
            ColorMapType::Heat,
            ColorMapType::Gray,
            ColorMapType::Rainbow,
            ColorMapType::BlueRed,
        ] {
            for i in 0..=100 {
                let t = i as f64 / 100.0;
                let cd = map_color(t, cmap);
                assert!(
                    cd.density <= 8,
                    "density {} > 8 at t={} cmap={:?}",
                    cd.density,
                    t,
                    cmap
                );
            }
        }
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

    #[test]
    fn blue_red_diverging() {
        let low = map_color(0.0, ColorMapType::BlueRed);
        assert_eq!(low.color, TermColor::Blue);

        let high = map_color(1.0, ColorMapType::BlueRed);
        assert_eq!(high.color, TermColor::Red);

        // Mid should have low density
        let mid = map_color(0.5, ColorMapType::BlueRed);
        assert!(mid.density <= 2);
    }
}
