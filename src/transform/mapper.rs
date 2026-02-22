/// Maps between data coordinates and pixel coordinates.
///
/// Handles normalization (data → 0.0–1.0), scaling to pixel space,
/// and Y-axis inversion for terminal coordinate systems.
pub struct CoordinateMapper {
    pub data_x_min: f64,
    pub data_x_max: f64,
    pub data_y_min: f64,
    pub data_y_max: f64,
    pub pixel_width: f64,
    pub pixel_height: f64,
}

impl CoordinateMapper {
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
            pixel_width: pixel_width as f64,
            pixel_height: pixel_height as f64,
        }
    }

    /// Converts data coordinates to pixel coordinates.
    /// Y is inverted so that higher data values map to lower pixel rows.
    pub fn data_to_pixel(&self, x: f64, y: f64) -> (f64, f64) {
        let x_range = self.data_x_max - self.data_x_min;
        let y_range = self.data_y_max - self.data_y_min;

        let norm_x = if x_range.abs() < f64::EPSILON {
            0.5
        } else {
            (x - self.data_x_min) / x_range
        };
        let norm_y = if y_range.abs() < f64::EPSILON {
            0.5
        } else {
            (y - self.data_y_min) / y_range
        };

        let px = norm_x * (self.pixel_width - 1.0);
        let py = (1.0 - norm_y) * (self.pixel_height - 1.0); // Y inverted
        (px, py)
    }

    /// Converts pixel coordinates back to data coordinates.
    pub fn pixel_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        let norm_x = px / (self.pixel_width - 1.0);
        let norm_y = 1.0 - py / (self.pixel_height - 1.0);

        let x = self.data_x_min + norm_x * (self.data_x_max - self.data_x_min);
        let y = self.data_y_min + norm_y * (self.data_y_max - self.data_y_min);
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_corners() {
        let m = CoordinateMapper::new(0.0, 10.0, 0.0, 10.0, 11, 11);
        // Bottom-left data → top-left pixel (Y inverted)
        let (px, py) = m.data_to_pixel(0.0, 0.0);
        assert!((px - 0.0).abs() < 1e-9);
        assert!((py - 10.0).abs() < 1e-9);
        // Top-right data → bottom-right pixel
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
        // Higher Y data value → lower pixel Y
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
}
