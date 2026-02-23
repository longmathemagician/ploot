/// 3D-to-2D projection via rotation matrix.
///
/// Builds a rotation matrix from azimuth and elevation angles,
/// then projects 3D points to 2D screen coordinates with depth.
pub struct Projection {
    /// 3x3 rotation matrix, row-major.
    rotation: [f64; 9],
}

impl Projection {
    /// Create a projection from azimuth and elevation angles in degrees.
    ///
    /// - `azimuth`: rotation around the Z axis (0 = looking along +X)
    /// - `elevation`: rotation above the XY plane (0 = side view, 90 = top-down)
    pub fn new(azimuth_deg: f64, elevation_deg: f64) -> Self {
        let az = azimuth_deg.to_radians();
        let el = elevation_deg.to_radians();

        let cos_az = az.cos();
        let sin_az = az.sin();
        let cos_el = el.cos();
        let sin_el = el.sin();

        // R = R_x(elevation) * R_z(azimuth)
        // R_z = [cos -sin 0; sin cos 0; 0 0 1]
        // R_x = [1 0 0; 0 cos -sin; 0 sin cos]
        let rotation = [
            cos_az,
            -sin_az,
            0.0,
            sin_az * cos_el,
            cos_az * cos_el,
            -sin_el,
            sin_az * sin_el,
            cos_az * sin_el,
            cos_el,
        ];

        Self { rotation }
    }

    /// Project a 3D point to (screen_x, screen_y, depth).
    ///
    /// The depth value is used for z-buffer testing (lower = closer).
    pub fn project(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let r = &self.rotation;
        let sx = r[0] * x + r[1] * y + r[2] * z;
        let sy = r[3] * x + r[4] * y + r[5] * z;
        let sz = r[6] * x + r[7] * y + r[8] * z;

        // Screen X = sx, Screen Y = -sy (Y up on screen), Depth = sz
        (sx, -sy, sz)
    }

    /// Project the 8 corners of a bounding box and return the 2D extent.
    ///
    /// Returns `(x_min, x_max, y_min, y_max)` in screen coordinates.
    pub fn projected_bounds(
        &self,
        x_range: (f64, f64),
        y_range: (f64, f64),
        z_range: (f64, f64),
    ) -> (f64, f64, f64, f64) {
        let mut sx_min = f64::INFINITY;
        let mut sx_max = f64::NEG_INFINITY;
        let mut sy_min = f64::INFINITY;
        let mut sy_max = f64::NEG_INFINITY;

        let xs = [x_range.0, x_range.1];
        let ys = [y_range.0, y_range.1];
        let zs = [z_range.0, z_range.1];

        for &x in &xs {
            for &y in &ys {
                for &z in &zs {
                    let (sx, sy, _) = self.project(x, y, z);
                    sx_min = sx_min.min(sx);
                    sx_max = sx_max.max(sx);
                    sy_min = sy_min.min(sy);
                    sy_max = sy_max.max(sy);
                }
            }
        }

        (sx_min, sx_max, sy_min, sy_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_rotation() {
        // azimuth=0, elevation=0: looking along +X
        let p = Projection::new(0.0, 0.0);
        let (sx, sy, depth) = p.project(1.0, 0.0, 0.0);
        // With az=0, el=0: sx = x, sy = -y, depth = z (roughly)
        assert!((sx - 1.0).abs() < 1e-9);
        assert!(sy.abs() < 1e-9);
        assert!(depth.abs() < 1e-9);
    }

    #[test]
    fn azimuth_90_rotates() {
        let p = Projection::new(90.0, 0.0);
        // Point on +X axis should appear on -Y screen axis (but projected to -sx)
        let (sx, sy, _) = p.project(1.0, 0.0, 0.0);
        // After 90 degree azimuth rotation:
        // sx = cos(90)*1 - sin(90)*0 = 0
        // sy = -(sin(90)*cos(0)*1 + cos(90)*cos(0)*0) = -1
        assert!(sx.abs() < 1e-9);
        assert!((sy - (-1.0)).abs() < 1e-9);
    }

    #[test]
    fn elevation_90_top_down() {
        let p = Projection::new(0.0, 90.0);
        // Point at (0, 0, 1) should project to screen
        let (sx, sy, _) = p.project(0.0, 0.0, 1.0);
        // el=90: looking from top
        // sy = -(sin(0)*cos(90)*0 + cos(0)*cos(90)*0 - sin(90)*1) = sin(90) = 1
        assert!((sy - 1.0).abs() < 1e-9);
        assert!(sx.abs() < 1e-9);
    }

    #[test]
    fn unit_cube_bounds() {
        let p = Projection::new(30.0, 30.0);
        let (sx_min, sx_max, sy_min, sy_max) =
            p.projected_bounds((-0.5, 0.5), (-0.5, 0.5), (-0.5, 0.5));
        // Bounds should be roughly symmetric
        assert!(sx_min < 0.0);
        assert!(sx_max > 0.0);
        assert!(sy_min < 0.0);
        assert!(sy_max > 0.0);
    }
}
