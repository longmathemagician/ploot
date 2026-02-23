use crate::api::grid::GridData;

/// A line segment from a contour at a specific level.
#[derive(Debug, Clone)]
pub struct ContourSegment {
    /// Start x coordinate (data space).
    pub x0: f64,
    /// Start y coordinate (data space).
    pub y0: f64,
    /// End x coordinate (data space).
    pub x1: f64,
    /// End y coordinate (data space).
    pub y1: f64,
    /// The contour level.
    pub level: f64,
}

/// Run marching squares to extract contour line segments at a given level.
pub fn marching_squares(grid: &GridData, level: f64) -> Vec<ContourSegment> {
    if grid.nx() < 2 || grid.ny() < 2 {
        return Vec::new();
    }

    let mut segments = Vec::new();

    let dx = (grid.x_max - grid.x_min) / (grid.nx() - 1) as f64;
    let dy = (grid.y_max - grid.y_min) / (grid.ny() - 1) as f64;

    for row in 0..grid.ny() - 1 {
        for col in 0..grid.nx() - 1 {
            let z_bl = grid.z_at(row, col); // bottom-left
            let z_br = grid.z_at(row, col + 1); // bottom-right
            let z_tl = grid.z_at(row + 1, col); // top-left
            let z_tr = grid.z_at(row + 1, col + 1); // top-right

            // Cell corners in data space
            let x_left = grid.x_min + col as f64 * dx;
            let x_right = grid.x_min + (col + 1) as f64 * dx;
            let y_bottom = grid.y_min + row as f64 * dy;
            let y_top = grid.y_min + (row + 1) as f64 * dy;

            // Classify corners: 1 if >= level, 0 if < level
            let case = ((z_bl >= level) as u8)
                | (((z_br >= level) as u8) << 1)
                | (((z_tr >= level) as u8) << 2)
                | (((z_tl >= level) as u8) << 3);

            // Edge interpolation helpers
            let bottom = || interp_x(x_left, x_right, z_bl, z_br, level);
            let right = || interp_y(y_bottom, y_top, z_br, z_tr, level);
            let top = || interp_x(x_left, x_right, z_tl, z_tr, level);
            let left = || interp_y(y_bottom, y_top, z_bl, z_tl, level);

            let b_y = y_bottom;
            let t_y = y_top;
            let l_x = x_left;
            let r_x = x_right;

            match case {
                0 | 15 => {} // no contour
                1 => {
                    // bottom-left above
                    segments.push(seg(bottom(), b_y, l_x, left(), level));
                }
                2 => {
                    // bottom-right above
                    segments.push(seg(bottom(), b_y, r_x, right(), level));
                }
                3 => {
                    // bottom edge above
                    segments.push(seg(l_x, left(), r_x, right(), level));
                }
                4 => {
                    // top-right above
                    segments.push(seg(r_x, right(), top(), t_y, level));
                }
                5 => {
                    // Saddle: bottom-left + top-right
                    let center = (z_bl + z_br + z_tl + z_tr) / 4.0;
                    if center >= level {
                        segments.push(seg(bottom(), b_y, r_x, right(), level));
                        segments.push(seg(l_x, left(), top(), t_y, level));
                    } else {
                        segments.push(seg(bottom(), b_y, l_x, left(), level));
                        segments.push(seg(r_x, right(), top(), t_y, level));
                    }
                }
                6 => {
                    // right edge above
                    segments.push(seg(bottom(), b_y, top(), t_y, level));
                }
                7 => {
                    // all but top-left
                    segments.push(seg(l_x, left(), top(), t_y, level));
                }
                8 => {
                    // top-left above
                    segments.push(seg(l_x, left(), top(), t_y, level));
                }
                9 => {
                    // left edge above
                    segments.push(seg(bottom(), b_y, top(), t_y, level));
                }
                10 => {
                    // Saddle: bottom-right + top-left
                    let center = (z_bl + z_br + z_tl + z_tr) / 4.0;
                    if center >= level {
                        segments.push(seg(bottom(), b_y, l_x, left(), level));
                        segments.push(seg(r_x, right(), top(), t_y, level));
                    } else {
                        segments.push(seg(bottom(), b_y, r_x, right(), level));
                        segments.push(seg(l_x, left(), top(), t_y, level));
                    }
                }
                11 => {
                    // all but top-right
                    segments.push(seg(r_x, right(), top(), t_y, level));
                }
                12 => {
                    // top edge above
                    segments.push(seg(l_x, left(), r_x, right(), level));
                }
                13 => {
                    // all but bottom-right
                    segments.push(seg(bottom(), b_y, r_x, right(), level));
                }
                14 => {
                    // all but bottom-left
                    segments.push(seg(bottom(), b_y, l_x, left(), level));
                }
                _ => unreachable!(),
            }
        }
    }

    segments
}

/// Generate evenly spaced contour levels between z_min and z_max.
pub fn auto_contour_levels(z_min: f64, z_max: f64, n_levels: usize) -> Vec<f64> {
    if n_levels == 0 {
        return Vec::new();
    }
    let mut levels = Vec::with_capacity(n_levels);
    for i in 0..n_levels {
        let t = (i as f64 + 1.0) / (n_levels as f64 + 1.0);
        levels.push(z_min + t * (z_max - z_min));
    }
    levels
}

fn seg(x0: f64, y0: f64, x1: f64, y1: f64, level: f64) -> ContourSegment {
    ContourSegment {
        x0,
        y0,
        x1,
        y1,
        level,
    }
}

fn interp_x(x_left: f64, x_right: f64, z_left: f64, z_right: f64, level: f64) -> f64 {
    let dz = z_right - z_left;
    if dz.abs() < f64::EPSILON {
        (x_left + x_right) / 2.0
    } else {
        x_left + (level - z_left) / dz * (x_right - x_left)
    }
}

fn interp_y(y_bottom: f64, y_top: f64, z_bottom: f64, z_top: f64, level: f64) -> f64 {
    let dz = z_top - z_bottom;
    if dz.abs() < f64::EPSILON {
        (y_bottom + y_top) / 2.0
    } else {
        y_bottom + (level - z_bottom) / dz * (y_top - y_bottom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_16_cases() {
        // Create a grid where we can control each corner
        for case in 0..16u8 {
            let z_bl = if case & 1 != 0 { 1.0 } else { 0.0 };
            let z_br = if case & 2 != 0 { 1.0 } else { 0.0 };
            let z_tr = if case & 4 != 0 { 1.0 } else { 0.0 };
            let z_tl = if case & 8 != 0 { 1.0 } else { 0.0 };

            let grid = GridData::from_rows(
                vec![vec![z_bl, z_br], vec![z_tl, z_tr]],
                (0.0, 1.0),
                (0.0, 1.0),
            );
            let segs = marching_squares(&grid, 0.5);

            match case {
                0 | 15 => assert!(segs.is_empty(), "case {case}: expected no segments"),
                5 | 10 => assert_eq!(segs.len(), 2, "case {case}: expected 2 segments (saddle)"),
                _ => assert_eq!(segs.len(), 1, "case {case}: expected 1 segment"),
            }
        }
    }

    #[test]
    fn tilted_plane_parallel_lines() {
        // z = x on a 10x10 grid
        let grid = GridData::from_fn(|x, _y| x, (0.0, 1.0), (0.0, 1.0), 10, 10);
        let levels = auto_contour_levels(0.0, 1.0, 5);

        for level in &levels {
            let segs = marching_squares(&grid, *level);
            assert!(!segs.is_empty(), "level {level} should have segments");
            // All segments should be roughly vertical (constant x)
            for seg in &segs {
                assert!(
                    (seg.x0 - seg.x1).abs() < 0.2,
                    "segment should be nearly vertical at level {level}"
                );
            }
        }
    }

    #[test]
    fn auto_levels_count() {
        let levels = auto_contour_levels(0.0, 10.0, 5);
        assert_eq!(levels.len(), 5);
        for l in &levels {
            assert!(*l > 0.0 && *l < 10.0);
        }
    }

    #[test]
    fn auto_levels_sorted() {
        let levels = auto_contour_levels(-5.0, 5.0, 8);
        for pair in levels.windows(2) {
            assert!(pair[0] < pair[1]);
        }
    }
}
