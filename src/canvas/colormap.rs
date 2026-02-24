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
    /// Density in [0.0, 1.0] representing sub-pixel fill fraction.
    pub density: f64,
}

/// The heat colormap band definitions: (color, start_t, end_t).
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
///
/// Density is a global monotonic ramp derived from `t`, not per-band.
pub fn map_color(t: f64, cmap: ColorMapType) -> ColorDensity {
    let t = t.clamp(0.0, 1.0);

    match cmap {
        ColorMapType::Heat => {
            let color = band_color(t, &HEAT_BANDS);
            ColorDensity {
                color,
                density: t * 0.875 + 0.125,
            }
        }
        ColorMapType::Gray => ColorDensity {
            color: TermColor::White,
            density: t,
        },
        ColorMapType::Rainbow => {
            let color = band_color(t, &RAINBOW_BANDS);
            ColorDensity {
                color,
                density: t * 0.875 + 0.125,
            }
        }
        ColorMapType::BlueRed => {
            let color = if t < 0.5 {
                TermColor::Blue
            } else {
                TermColor::Red
            };
            // V-shape: 0 at midpoint, 1 at extremes
            let density = (t - 0.5).abs() * 2.0;
            ColorDensity { color, density }
        }
    }
}

/// Look up the band color for a given `t` value.
fn band_color(t: f64, bands: &[(TermColor, f64, f64)]) -> TermColor {
    for &(color, start, end) in bands {
        if t >= start && t <= end + f64::EPSILON {
            return color;
        }
    }
    // Fallback for t == 1.0
    bands[bands.len() - 1].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_map_endpoints() {
        let low = map_color(0.0, ColorMapType::Heat);
        assert_eq!(low.color, TermColor::Blue);
        assert!((low.density - 0.125).abs() < 0.01);

        let high = map_color(1.0, ColorMapType::Heat);
        assert_eq!(high.color, TermColor::White);
        assert!((high.density - 1.0).abs() < 0.01);
    }

    #[test]
    fn gray_map_range() {
        let low = map_color(0.0, ColorMapType::Gray);
        assert!((low.density - 0.0).abs() < 0.01);

        let high = map_color(1.0, ColorMapType::Gray);
        assert!((high.density - 1.0).abs() < 0.01);
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
                    cd.density >= 0.0 && cd.density <= 1.0,
                    "density {} out of [0,1] at t={} cmap={:?}",
                    cd.density,
                    t,
                    cmap
                );
            }
        }
    }

    #[test]
    fn blue_red_diverging() {
        let low = map_color(0.0, ColorMapType::BlueRed);
        assert_eq!(low.color, TermColor::Blue);

        let high = map_color(1.0, ColorMapType::BlueRed);
        assert_eq!(high.color, TermColor::Red);

        // Mid should have near-zero density
        let mid = map_color(0.5, ColorMapType::BlueRed);
        assert!(mid.density < 0.05);
    }

    #[test]
    fn heat_density_monotonic() {
        let mut prev = 0.0_f64;
        for i in 0..=1000 {
            let t = i as f64 / 1000.0;
            let cd = map_color(t, ColorMapType::Heat);
            assert!(
                cd.density >= prev - f64::EPSILON,
                "density decreased at t={}: {} < {}",
                t,
                cd.density,
                prev
            );
            prev = cd.density;
        }
    }
}
