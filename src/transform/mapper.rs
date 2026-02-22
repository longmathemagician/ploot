/// Maps between data coordinates and pixel coordinates.
///
/// Handles normalization (data -> 0.0-1.0), scaling to pixel space,
/// Y-axis inversion for terminal coordinate systems, and optional
/// axis reversal and log scaling.
pub struct CoordinateMapper {
    /// Minimum data x value.
    pub data_x_min: f64,
    /// Maximum data x value.
    pub data_x_max: f64,
    /// Minimum data y value.
    pub data_y_min: f64,
    /// Maximum data y value.
    pub data_y_max: f64,
    /// Minimum pixel x coordinate.
    x_pixel_min: f64,
    /// Maximum pixel x coordinate.
    x_pixel_max: f64,
    /// Minimum pixel y coordinate.
    y_pixel_min: f64,
    /// Maximum pixel y coordinate.
    y_pixel_max: f64,
    /// Whether the x-axis is reversed.
    pub x_reversed: bool,
    /// Whether the y-axis is reversed.
    pub y_reversed: bool,
    /// Log base for x-axis (e.g. `Some(10.0)`), or `None` for linear.
    pub x_log_base: Option<f64>,
    /// Log base for y-axis (e.g. `Some(10.0)`), or `None` for linear.
    pub y_log_base: Option<f64>,
}

impl CoordinateMapper {
    /// Create a new linear coordinate mapper for the given data and pixel ranges.
    ///
    /// Pixel range spans from 0 to `(pixel_width - 1)` and `(pixel_height - 1)`
    /// respectively, matching the old behaviour exactly.
    pub fn new(
        data_x_min: f64,
        data_x_max: f64,
        data_y_min: f64,
        data_y_max: f64,
        pixel_width: usize,
        pixel_height: usize,
    ) -> Self {
        Self {
            data_x_min,
            data_x_max,
            data_y_min,
            data_y_max,
            x_pixel_min: 0.0,
            x_pixel_max: (pixel_width as f64 - 1.0).max(0.0),
            y_pixel_min: 0.0,
            y_pixel_max: (pixel_height as f64 - 1.0).max(0.0),
            x_reversed: false,
            y_reversed: false,
            x_log_base: None,
            y_log_base: None,
        }
    }

    /// Create a mapper with explicit pixel-range endpoints.
    ///
    /// Use this when you need tick values to land on exact sub-pixel
    /// positions (e.g. cell-aligned grids).
    #[allow(clippy::too_many_arguments)]
    pub fn with_pixel_ranges(
        data_x_min: f64,
        data_x_max: f64,
        data_y_min: f64,
        data_y_max: f64,
        x_pixel_min: f64,
        x_pixel_max: f64,
        y_pixel_min: f64,
        y_pixel_max: f64,
    ) -> Self {
        Self {
            data_x_min,
            data_x_max,
            data_y_min,
            data_y_max,
            x_pixel_min,
            x_pixel_max,
            y_pixel_min,
            y_pixel_max,
            x_reversed: false,
            y_reversed: false,
            x_log_base: None,
            y_log_base: None,
        }
    }

    /// Set axis reversal flags.
    pub fn with_reversal(mut self, x_reversed: bool, y_reversed: bool) -> Self {
        self.x_reversed = x_reversed;
        self.y_reversed = y_reversed;
        self
    }

    /// Set log scale bases.
    pub fn with_log(mut self, x_log_base: Option<f64>, y_log_base: Option<f64>) -> Self {
        self.x_log_base = x_log_base;
        self.y_log_base = y_log_base;
        self
    }

    /// Apply log transformation to a value.
    fn log_transform(value: f64, base: f64) -> f64 {
        if value <= 0.0 {
            f64::NEG_INFINITY
        } else {
            value.log(base)
        }
    }

    /// Converts data coordinates to pixel coordinates.
    /// Y is inverted so that higher data values map to lower pixel rows.
    pub fn data_to_pixel(&self, x: f64, y: f64) -> (f64, f64) {
        let (x_val, x_min, x_max) = if let Some(base) = self.x_log_base {
            (
                Self::log_transform(x, base),
                Self::log_transform(self.data_x_min, base),
                Self::log_transform(self.data_x_max, base),
            )
        } else {
            (x, self.data_x_min, self.data_x_max)
        };

        let (y_val, y_min, y_max) = if let Some(base) = self.y_log_base {
            (
                Self::log_transform(y, base),
                Self::log_transform(self.data_y_min, base),
                Self::log_transform(self.data_y_max, base),
            )
        } else {
            (y, self.data_y_min, self.data_y_max)
        };

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        let mut norm_x = if x_range.abs() < f64::EPSILON {
            0.5
        } else {
            (x_val - x_min) / x_range
        };
        let mut norm_y = if y_range.abs() < f64::EPSILON {
            0.5
        } else {
            (y_val - y_min) / y_range
        };

        if self.x_reversed {
            norm_x = 1.0 - norm_x;
        }
        if self.y_reversed {
            norm_y = 1.0 - norm_y;
        }

        let px = self.x_pixel_min + norm_x * (self.x_pixel_max - self.x_pixel_min);
        let py = self.y_pixel_min + (1.0 - norm_y) * (self.y_pixel_max - self.y_pixel_min);
        (px, py)
    }

    /// Converts pixel coordinates back to data coordinates.
    pub fn pixel_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        let x_pixel_range = self.x_pixel_max - self.x_pixel_min;
        let y_pixel_range = self.y_pixel_max - self.y_pixel_min;

        let mut norm_x = if x_pixel_range.abs() < f64::EPSILON {
            0.5
        } else {
            (px - self.x_pixel_min) / x_pixel_range
        };
        let mut norm_y = if y_pixel_range.abs() < f64::EPSILON {
            0.5
        } else {
            1.0 - (py - self.y_pixel_min) / y_pixel_range
        };

        if self.x_reversed {
            norm_x = 1.0 - norm_x;
        }
        if self.y_reversed {
            norm_y = 1.0 - norm_y;
        }

        let x = self.data_x_min + norm_x * (self.data_x_max - self.data_x_min);
        let y = self.data_y_min + norm_y * (self.data_y_max - self.data_y_min);
        (x, y)
    }
}

/// Compute cell-aligned y pixel range for a given canvas height and tick count.
///
/// Returns `(y_pixel_min, y_pixel_max)` where tick_max maps to `y_pixel_min`
/// (top of canvas, sub-pixel row 1 of cell 0) and tick_min maps to
/// `y_pixel_max` (bottom, sub-pixel row 1 of the last aligned cell).
pub fn aligned_y_pixel_range(canvas_char_height: usize, n_ticks: usize) -> (f64, f64) {
    if n_ticks <= 1 {
        // Center single tick
        let mid = (canvas_char_height as f64 * 4.0 - 1.0) / 2.0;
        return (mid, mid);
    }
    let cells_per_interval = ((canvas_char_height - 1) / (n_ticks - 1)).max(1);
    let y_pixel_min = 1.0; // sub-pixel row 1 of cell 0
    let y_pixel_max = (1 + (n_ticks - 1) * cells_per_interval * 4) as f64;
    (y_pixel_min, y_pixel_max)
}

/// Compute cell-aligned x pixel range for a given canvas width and tick count.
///
/// Returns `(x_pixel_min, x_pixel_max)` where tick_min maps to `x_pixel_min`
/// (sub-pixel column 0 of cell 0) and tick_max maps to `x_pixel_max`.
pub fn aligned_x_pixel_range(canvas_char_width: usize, n_ticks: usize) -> (f64, f64) {
    if n_ticks <= 1 {
        let mid = (canvas_char_width as f64 * 2.0 - 1.0) / 2.0;
        return (mid, mid);
    }
    let cells_per_interval = ((canvas_char_width - 1) / (n_ticks - 1)).max(1);
    let x_pixel_min = 0.0;
    let x_pixel_max = ((n_ticks - 1) * cells_per_interval * 2) as f64;
    (x_pixel_min, x_pixel_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_corners() {
        let m = CoordinateMapper::new(0.0, 10.0, 0.0, 10.0, 11, 11);
        // Bottom-left data -> top-left pixel (Y inverted)
        let (px, py) = m.data_to_pixel(0.0, 0.0);
        assert!((px - 0.0).abs() < 1e-9);
        assert!((py - 10.0).abs() < 1e-9);
        // Top-right data -> bottom-right pixel
        let (px, py) = m.data_to_pixel(10.0, 10.0);
        assert!((px - 10.0).abs() < 1e-9);
        assert!((py - 0.0).abs() < 1e-9);
    }

    #[test]
    fn negative_range() {
        let m = CoordinateMapper::new(-5.0, 5.0, -10.0, 10.0, 101, 201);
        let (px, py) = m.data_to_pixel(0.0, 0.0);
        assert!((px - 50.0).abs() < 1e-9);
        assert!((py - 100.0).abs() < 1e-9);
    }

    #[test]
    fn y_inversion() {
        let m = CoordinateMapper::new(0.0, 100.0, 0.0, 100.0, 101, 101);
        // Higher Y data value -> lower pixel Y
        let (_, py_high) = m.data_to_pixel(50.0, 100.0);
        let (_, py_low) = m.data_to_pixel(50.0, 0.0);
        assert!(py_high < py_low);
    }

    #[test]
    fn roundtrip() {
        let m = CoordinateMapper::new(-3.0, 3.0, -10.0, 10.0, 161, 81);
        let (px, py) = m.data_to_pixel(1.5, -2.5);
        let (x, y) = m.pixel_to_data(px, py);
        assert!((x - 1.5).abs() < 1e-9);
        assert!((y - (-2.5)).abs() < 1e-9);
    }

    #[test]
    fn zero_range_centers() {
        let m = CoordinateMapper::new(5.0, 5.0, 5.0, 5.0, 101, 101);
        let (px, py) = m.data_to_pixel(5.0, 5.0);
        assert!((px - 50.0).abs() < 1e-9);
        assert!((py - 50.0).abs() < 1e-9);
    }

    #[test]
    fn x_reversed() {
        let m = CoordinateMapper::new(0.0, 10.0, 0.0, 10.0, 11, 11).with_reversal(true, false);
        // x=0 should map to the right (px=10), x=10 to the left (px=0)
        let (px, _) = m.data_to_pixel(0.0, 5.0);
        assert!((px - 10.0).abs() < 1e-9);
        let (px, _) = m.data_to_pixel(10.0, 5.0);
        assert!((px - 0.0).abs() < 1e-9);
    }

    #[test]
    fn y_reversed() {
        let m = CoordinateMapper::new(0.0, 10.0, 0.0, 10.0, 11, 11).with_reversal(false, true);
        // y=10 should map to the bottom (py=10), y=0 to the top (py=0)
        let (_, py) = m.data_to_pixel(5.0, 10.0);
        assert!((py - 10.0).abs() < 1e-9);
        let (_, py) = m.data_to_pixel(5.0, 0.0);
        assert!((py - 0.0).abs() < 1e-9);
    }

    #[test]
    fn with_pixel_ranges_basic() {
        let m = CoordinateMapper::with_pixel_ranges(0.0, 10.0, 0.0, 10.0, 5.0, 15.0, 2.0, 22.0);
        // data_max_y -> y_pixel_min (top), data_min_y -> y_pixel_max (bottom)
        let (px, py) = m.data_to_pixel(0.0, 10.0);
        assert!((px - 5.0).abs() < 1e-9);
        assert!((py - 2.0).abs() < 1e-9);
        let (px, py) = m.data_to_pixel(10.0, 0.0);
        assert!((px - 15.0).abs() < 1e-9);
        assert!((py - 22.0).abs() < 1e-9);
    }

    #[test]
    fn with_pixel_ranges_roundtrip() {
        let m = CoordinateMapper::with_pixel_ranges(0.0, 10.0, 0.0, 20.0, 1.0, 41.0, 1.0, 81.0);
        let (px, py) = m.data_to_pixel(5.0, 10.0);
        let (x, y) = m.pixel_to_data(px, py);
        assert!((x - 5.0).abs() < 1e-9);
        assert!((y - 10.0).abs() < 1e-9);
    }

    #[test]
    fn aligned_y_range_ticks_hit_cell_row_1() {
        // 5 ticks, 20 cells high => cells_per_interval = 4
        let (y_min, y_max) = aligned_y_pixel_range(20, 5);
        assert!((y_min - 1.0).abs() < 1e-9);
        // y_max = 1 + 4 * 4 * 4 = 1 + 64 = 65
        assert!((y_max - 65.0).abs() < 1e-9);

        // Each tick should land at sub-pixel row 1 of its cell
        let m = CoordinateMapper::with_pixel_ranges(0.0, 4.0, 0.0, 4.0, 0.0, 10.0, y_min, y_max);
        for i in 0..5 {
            let val = i as f64;
            let (_, py) = m.data_to_pixel(0.0, val);
            let pixel_row = py.round() as usize;
            assert_eq!(pixel_row % 4, 1, "tick {i} at py={py} not cell-aligned");
        }
    }

    #[test]
    fn aligned_x_range_ticks_hit_cell_boundaries() {
        // 6 ticks, 30 cells wide => cells_per_interval = 5
        let (x_min, x_max) = aligned_x_pixel_range(30, 6);
        assert!((x_min - 0.0).abs() < 1e-9);
        // x_max = 5 * 5 * 2 = 50
        assert!((x_max - 50.0).abs() < 1e-9);
    }

    #[test]
    fn aligned_single_tick() {
        let (y_min, y_max) = aligned_y_pixel_range(10, 1);
        assert!((y_min - y_max).abs() < 1e-9);
        let (x_min, x_max) = aligned_x_pixel_range(10, 1);
        assert!((x_min - x_max).abs() < 1e-9);
    }
}
