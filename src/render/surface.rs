use crate::api::grid::GridData;
use crate::canvas::BrailleCanvas;
use crate::canvas::SOLID;
use crate::canvas::color::TermColor;
use crate::canvas::colormap::{ColorMapType, map_color};
use crate::canvas::depth::DepthCanvas;
use crate::transform::projection::Projection;

/// Draw a wireframe surface (all row and column mesh lines, no occlusion).
#[allow(clippy::too_many_arguments)]
pub fn draw_surface_wireframe(
    canvas: &mut BrailleCanvas,
    grid: &GridData,
    projection: &Projection,
    scale_x: f64,
    scale_y: f64,
    offset_x: f64,
    offset_y: f64,
    color: TermColor,
) {
    let nx = grid.nx();
    let ny = grid.ny();
    if nx < 2 || ny < 2 {
        return;
    }

    let norm = |grid: &GridData, row: usize, col: usize| -> (f64, f64, f64) {
        let x = if nx > 1 {
            (col as f64 / (nx - 1) as f64) - 0.5
        } else {
            0.0
        };
        let y = if ny > 1 {
            (row as f64 / (ny - 1) as f64) - 0.5
        } else {
            0.0
        };
        let z = grid.normalized_z(grid.z_at(row, col)) - 0.5;
        (x, y, z)
    };

    // Draw row lines
    for row in 0..ny {
        for col in 0..nx - 1 {
            let (x0, y0, z0) = norm(grid, row, col);
            let (x1, y1, z1) = norm(grid, row, col + 1);
            let (sx0, sy0, _) = projection.project(x0, y0, z0);
            let (sx1, sy1, _) = projection.project(x1, y1, z1);
            let px0 = (sx0 * scale_x + offset_x).round() as i32;
            let py0 = (-sy0 * scale_y + offset_y).round() as i32;
            let px1 = (sx1 * scale_x + offset_x).round() as i32;
            let py1 = (-sy1 * scale_y + offset_y).round() as i32;
            canvas.draw_line(px0, py0, px1, py1, color, &SOLID);
        }
    }

    // Draw column lines
    for col in 0..nx {
        for row in 0..ny - 1 {
            let (x0, y0, z0) = norm(grid, row, col);
            let (x1, y1, z1) = norm(grid, row + 1, col);
            let (sx0, sy0, _) = projection.project(x0, y0, z0);
            let (sx1, sy1, _) = projection.project(x1, y1, z1);
            let px0 = (sx0 * scale_x + offset_x).round() as i32;
            let py0 = (-sy0 * scale_y + offset_y).round() as i32;
            let px1 = (sx1 * scale_x + offset_x).round() as i32;
            let py1 = (-sy1 * scale_y + offset_y).round() as i32;
            canvas.draw_line(px0, py0, px1, py1, color, &SOLID);
        }
    }
}

/// Draw a wireframe surface with hidden-line removal via depth buffer.
#[allow(clippy::too_many_arguments)]
pub fn draw_surface_hidden(
    depth_canvas: &mut DepthCanvas,
    grid: &GridData,
    projection: &Projection,
    scale_x: f64,
    scale_y: f64,
    offset_x: f64,
    offset_y: f64,
    color: TermColor,
) {
    let nx = grid.nx();
    let ny = grid.ny();
    if nx < 2 || ny < 2 {
        return;
    }

    let norm = |grid: &GridData, row: usize, col: usize| -> (f64, f64, f64) {
        let x = (col as f64 / (nx - 1) as f64) - 0.5;
        let y = (row as f64 / (ny - 1) as f64) - 0.5;
        let z = grid.normalized_z(grid.z_at(row, col)) - 0.5;
        (x, y, z)
    };

    // Draw row lines with depth
    for row in 0..ny {
        for col in 0..nx - 1 {
            let (x0, y0, z0) = norm(grid, row, col);
            let (x1, y1, z1) = norm(grid, row, col + 1);
            let (sx0, sy0, d0) = projection.project(x0, y0, z0);
            let (sx1, sy1, d1) = projection.project(x1, y1, z1);
            let px0 = (sx0 * scale_x + offset_x).round() as i32;
            let py0 = (-sy0 * scale_y + offset_y).round() as i32;
            let px1 = (sx1 * scale_x + offset_x).round() as i32;
            let py1 = (-sy1 * scale_y + offset_y).round() as i32;
            depth_canvas.draw_line_depth(px0, py0, d0, px1, py1, d1, color);
        }
    }

    // Draw column lines with depth
    for col in 0..nx {
        for row in 0..ny - 1 {
            let (x0, y0, z0) = norm(grid, row, col);
            let (x1, y1, z1) = norm(grid, row + 1, col);
            let (sx0, sy0, d0) = projection.project(x0, y0, z0);
            let (sx1, sy1, d1) = projection.project(x1, y1, z1);
            let px0 = (sx0 * scale_x + offset_x).round() as i32;
            let py0 = (-sy0 * scale_y + offset_y).round() as i32;
            let px1 = (sx1 * scale_x + offset_x).round() as i32;
            let py1 = (-sy1 * scale_y + offset_y).round() as i32;
            depth_canvas.draw_line_depth(px0, py0, d0, px1, py1, d1, color);
        }
    }
}

/// Draw a filled surface with color-mapped density shading.
#[allow(clippy::too_many_arguments)]
pub fn draw_surface_filled(
    depth_canvas: &mut DepthCanvas,
    grid: &GridData,
    projection: &Projection,
    scale_x: f64,
    scale_y: f64,
    offset_x: f64,
    offset_y: f64,
    colormap: ColorMapType,
) {
    let nx = grid.nx();
    let ny = grid.ny();
    if nx < 2 || ny < 2 {
        return;
    }

    // For each grid cell, fill with color-mapped density
    for row in 0..ny - 1 {
        for col in 0..nx - 1 {
            // Average z for this quad
            let z_avg = (grid.z_at(row, col)
                + grid.z_at(row, col + 1)
                + grid.z_at(row + 1, col)
                + grid.z_at(row + 1, col + 1))
                / 4.0;
            let t = grid.normalized_z(z_avg);
            let cd = map_color(t, colormap);

            // Center of the quad in normalized coords
            let cx = ((col as f64 + 0.5) / (nx - 1) as f64) - 0.5;
            let cy = ((row as f64 + 0.5) / (ny - 1) as f64) - 0.5;
            let cz = (grid.normalized_z(z_avg)) - 0.5;

            let (sx, sy, depth) = projection.project(cx, cy, cz);
            let px = (sx * scale_x + offset_x).round() as i32;
            let py = (-sy * scale_y + offset_y).round() as i32;

            // Map to character cell
            if px >= 0 && py >= 0 {
                let char_x = px as usize / 2;
                let char_y = py as usize / 4;
                depth_canvas.fill_cell_density(char_x, char_y, cd, depth);
            }
        }
    }

    // Draw wireframe edges on top for definition
    draw_surface_hidden(
        depth_canvas,
        grid,
        projection,
        scale_x,
        scale_y,
        offset_x,
        offset_y,
        TermColor::White,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wireframe_flat_plane() {
        let grid = GridData::from_fn(|_, _| 0.5, (0.0, 1.0), (0.0, 1.0), 5, 5);
        let proj = Projection::new(30.0, 30.0);
        let mut canvas = BrailleCanvas::new(20, 10);
        let pw = canvas.pixel_width() as f64;
        let ph = canvas.pixel_height() as f64;
        draw_surface_wireframe(
            &mut canvas,
            &grid,
            &proj,
            pw * 0.8,
            ph * 0.8,
            pw / 2.0,
            ph / 2.0,
            TermColor::Green,
        );
        let s = canvas.render_plain();
        let non_blank = s.chars().filter(|&c| c != '\u{2800}' && c != '\n').count();
        assert!(non_blank > 0);
    }

    #[test]
    fn hidden_line_flat_plane() {
        let grid = GridData::from_fn(|_, _| 0.5, (0.0, 1.0), (0.0, 1.0), 5, 5);
        let proj = Projection::new(30.0, 30.0);
        let mut dc = DepthCanvas::new(20, 10);
        let pw = dc.pixel_width() as f64;
        let ph = dc.pixel_height() as f64;
        draw_surface_hidden(
            &mut dc,
            &grid,
            &proj,
            pw * 0.8,
            ph * 0.8,
            pw / 2.0,
            ph / 2.0,
            TermColor::Green,
        );
        let s = dc.canvas().render_plain();
        let non_blank = s.chars().filter(|&c| c != '\u{2800}' && c != '\n').count();
        assert!(non_blank > 0);
    }

    #[test]
    fn filled_surface_produces_output() {
        let grid = GridData::from_fn(
            |x, y| (x * x + y * y).sqrt(),
            (0.0, 1.0),
            (0.0, 1.0),
            10,
            10,
        );
        let proj = Projection::new(30.0, 30.0);
        let mut dc = DepthCanvas::new(20, 10);
        let pw = dc.pixel_width() as f64;
        let ph = dc.pixel_height() as f64;
        draw_surface_filled(
            &mut dc,
            &grid,
            &proj,
            pw * 0.8,
            ph * 0.8,
            pw / 2.0,
            ph / 2.0,
            ColorMapType::Heat,
        );
        let s = dc.canvas().render_plain();
        let non_blank = s.chars().filter(|&c| c != '\u{2800}' && c != '\n').count();
        assert!(non_blank > 0);
    }
}
