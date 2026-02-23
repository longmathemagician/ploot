/// 2D grid data for heatmaps, contours, and 3D surfaces.
///
/// Stores Z(x,y) values on a regular grid with uniform spacing.
#[derive(Debug, Clone)]
pub struct GridData {
    /// Z values in row-major order (row 0 = y_min).
    pub(crate) z_values: Vec<f64>,
    /// Number of columns (x-direction).
    pub(crate) nx: usize,
    /// Number of rows (y-direction).
    pub(crate) ny: usize,
    /// Minimum x value.
    pub(crate) x_min: f64,
    /// Maximum x value.
    pub(crate) x_max: f64,
    /// Minimum y value.
    pub(crate) y_min: f64,
    /// Maximum y value.
    pub(crate) y_max: f64,
    /// Cached minimum z value.
    pub(crate) z_min: f64,
    /// Cached maximum z value.
    pub(crate) z_max: f64,
}

impl GridData {
    /// Create a grid from nested iterators of rows.
    ///
    /// Each inner iterator yields the z-values for one row (y = constant).
    /// Row 0 corresponds to `y_range.0`, and the last row to `y_range.1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::api::grid::GridData;
    ///
    /// let rows = vec![
    ///     vec![0.0, 1.0, 2.0],
    ///     vec![3.0, 4.0, 5.0],
    /// ];
    /// let grid = GridData::from_rows(rows, (0.0, 2.0), (0.0, 1.0));
    /// assert_eq!(grid.nx(), 3);
    /// assert_eq!(grid.ny(), 2);
    /// ```
    pub fn from_rows<I, R, S>(rows: I, x_range: (f64, f64), y_range: (f64, f64)) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let mut z_values = Vec::new();
        let mut ny = 0usize;
        let mut nx = 0usize;

        for row in rows {
            let row_data: Vec<f64> = row.into_iter().map(|v| v.into()).collect();
            if ny == 0 {
                nx = row_data.len();
            }
            z_values.extend_from_slice(&row_data);
            ny += 1;
        }

        let (z_min, z_max) = compute_z_range(&z_values);

        Self {
            z_values,
            nx,
            ny,
            x_min: x_range.0,
            x_max: x_range.1,
            y_min: y_range.0,
            y_max: y_range.1,
            z_min,
            z_max,
        }
    }

    /// Create a grid by sampling a function on a uniform grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use ploot::api::grid::GridData;
    ///
    /// let grid = GridData::from_fn(
    ///     |x, y| x * x + y * y,
    ///     (-1.0, 1.0),
    ///     (-1.0, 1.0),
    ///     10,
    ///     10,
    /// );
    /// assert_eq!(grid.nx(), 10);
    /// assert_eq!(grid.ny(), 10);
    /// ```
    pub fn from_fn(
        f: impl Fn(f64, f64) -> f64,
        x_range: (f64, f64),
        y_range: (f64, f64),
        nx: usize,
        ny: usize,
    ) -> Self {
        let mut z_values = Vec::with_capacity(nx * ny);

        for j in 0..ny {
            let y = if ny > 1 {
                y_range.0 + (y_range.1 - y_range.0) * j as f64 / (ny - 1) as f64
            } else {
                (y_range.0 + y_range.1) / 2.0
            };
            for i in 0..nx {
                let x = if nx > 1 {
                    x_range.0 + (x_range.1 - x_range.0) * i as f64 / (nx - 1) as f64
                } else {
                    (x_range.0 + x_range.1) / 2.0
                };
                z_values.push(f(x, y));
            }
        }

        let (z_min, z_max) = compute_z_range(&z_values);

        Self {
            z_values,
            nx,
            ny,
            x_min: x_range.0,
            x_max: x_range.1,
            y_min: y_range.0,
            y_max: y_range.1,
            z_min,
            z_max,
        }
    }

    /// Number of columns.
    pub fn nx(&self) -> usize {
        self.nx
    }

    /// Number of rows.
    pub fn ny(&self) -> usize {
        self.ny
    }

    /// Returns the z value at grid position (row, col).
    pub fn z_at(&self, row: usize, col: usize) -> f64 {
        self.z_values[row * self.nx + col]
    }

    /// Bilinear interpolation at arbitrary (x, y) in data coordinates.
    pub fn interpolate(&self, x: f64, y: f64) -> f64 {
        if self.nx < 2 || self.ny < 2 {
            return if self.z_values.is_empty() {
                0.0
            } else {
                self.z_values[0]
            };
        }

        // Map to fractional grid indices
        let fx = (x - self.x_min) / (self.x_max - self.x_min) * (self.nx - 1) as f64;
        let fy = (y - self.y_min) / (self.y_max - self.y_min) * (self.ny - 1) as f64;

        let ix = fx.floor().max(0.0) as usize;
        let iy = fy.floor().max(0.0) as usize;

        let ix = ix.min(self.nx - 2);
        let iy = iy.min(self.ny - 2);

        let tx = fx - ix as f64;
        let ty = fy - iy as f64;
        let tx = tx.clamp(0.0, 1.0);
        let ty = ty.clamp(0.0, 1.0);

        let z00 = self.z_at(iy, ix);
        let z10 = self.z_at(iy, ix + 1);
        let z01 = self.z_at(iy + 1, ix);
        let z11 = self.z_at(iy + 1, ix + 1);

        let z0 = z00 * (1.0 - tx) + z10 * tx;
        let z1 = z01 * (1.0 - tx) + z11 * tx;
        z0 * (1.0 - ty) + z1 * ty
    }

    /// Maps a z value to normalized [0, 1] range.
    pub fn normalized_z(&self, z: f64) -> f64 {
        let range = self.z_max - self.z_min;
        if range.abs() < f64::EPSILON {
            0.5
        } else {
            ((z - self.z_min) / range).clamp(0.0, 1.0)
        }
    }

    /// Minimum z value.
    pub fn z_min(&self) -> f64 {
        self.z_min
    }

    /// Maximum z value.
    pub fn z_max(&self) -> f64 {
        self.z_max
    }

    /// X range.
    pub fn x_range(&self) -> (f64, f64) {
        (self.x_min, self.x_max)
    }

    /// Y range.
    pub fn y_range(&self) -> (f64, f64) {
        (self.y_min, self.y_max)
    }
}

fn compute_z_range(z: &[f64]) -> (f64, f64) {
    let mut z_min = f64::INFINITY;
    let mut z_max = f64::NEG_INFINITY;
    for &v in z {
        if v.is_finite() {
            z_min = z_min.min(v);
            z_max = z_max.max(v);
        }
    }
    if !z_min.is_finite() || !z_max.is_finite() {
        z_min = 0.0;
        z_max = 1.0;
    }
    if (z_max - z_min).abs() < f64::EPSILON {
        z_min -= 0.5;
        z_max += 0.5;
    }
    (z_min, z_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rows_basic() {
        let rows = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let grid = GridData::from_rows(rows, (0.0, 2.0), (0.0, 1.0));
        assert_eq!(grid.nx(), 3);
        assert_eq!(grid.ny(), 2);
        assert_eq!(grid.z_at(0, 0), 1.0);
        assert_eq!(grid.z_at(1, 2), 6.0);
    }

    #[test]
    fn from_fn_basic() {
        let grid = GridData::from_fn(|x, y| x + y, (0.0, 1.0), (0.0, 1.0), 3, 3);
        assert_eq!(grid.nx(), 3);
        assert_eq!(grid.ny(), 3);
        assert!((grid.z_at(0, 0) - 0.0).abs() < 1e-9);
        assert!((grid.z_at(2, 2) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn z_range() {
        let grid =
            GridData::from_rows(vec![vec![1.0, 5.0], vec![3.0, 7.0]], (0.0, 1.0), (0.0, 1.0));
        assert!((grid.z_min() - 1.0).abs() < 1e-9);
        assert!((grid.z_max() - 7.0).abs() < 1e-9);
    }

    #[test]
    fn interpolate_corners() {
        let grid =
            GridData::from_rows(vec![vec![0.0, 1.0], vec![2.0, 3.0]], (0.0, 1.0), (0.0, 1.0));
        assert!((grid.interpolate(0.0, 0.0) - 0.0).abs() < 1e-9);
        assert!((grid.interpolate(1.0, 0.0) - 1.0).abs() < 1e-9);
        assert!((grid.interpolate(0.0, 1.0) - 2.0).abs() < 1e-9);
        assert!((grid.interpolate(1.0, 1.0) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn interpolate_center() {
        let grid =
            GridData::from_rows(vec![vec![0.0, 2.0], vec![2.0, 4.0]], (0.0, 1.0), (0.0, 1.0));
        let center = grid.interpolate(0.5, 0.5);
        assert!((center - 2.0).abs() < 1e-9);
    }

    #[test]
    fn normalized_z_basic() {
        let grid = GridData::from_rows(vec![vec![0.0, 10.0]], (0.0, 1.0), (0.0, 0.0));
        assert!((grid.normalized_z(0.0) - 0.0).abs() < 1e-9);
        assert!((grid.normalized_z(10.0) - 1.0).abs() < 1e-9);
        assert!((grid.normalized_z(5.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn normalized_z_clamps() {
        let grid = GridData::from_rows(vec![vec![0.0, 10.0]], (0.0, 1.0), (0.0, 0.0));
        assert!((grid.normalized_z(-5.0) - 0.0).abs() < 1e-9);
        assert!((grid.normalized_z(15.0) - 1.0).abs() < 1e-9);
    }
}
