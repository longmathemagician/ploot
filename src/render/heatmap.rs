use crate::api::grid::GridData;
use crate::canvas::BrailleCanvas;
use crate::canvas::colormap::{ColorMapType, DENSITY_FILL_ORDER, map_color};
use crate::transform::CoordinateMapper;

/// Draw a heatmap from grid data onto the canvas.
pub fn draw_heatmap(
    canvas: &mut BrailleCanvas,
    grid: &GridData,
    mapper: &CoordinateMapper,
    colormap: ColorMapType,
) {
    let cw = canvas.char_width();
    let ch = canvas.char_height();

    for cy in 0..ch {
        for cx in 0..cw {
            // Character cell center in pixel coords
            let px = cx as f64 * 2.0 + 1.0;
            let py = cy as f64 * 4.0 + 2.0;

            // Map pixel to data coords
            let (data_x, data_y) = mapper.pixel_to_data(px, py);

            // Check if within grid bounds
            if data_x < grid.x_min
                || data_x > grid.x_max
                || data_y < grid.y_min
                || data_y > grid.y_max
            {
                continue;
            }

            // Interpolate z value
            let z = grid.interpolate(data_x, data_y);
            let t = grid.normalized_z(z);

            // Map to color+density
            let cd = map_color(t, colormap);

            // Fill the appropriate sub-pixels
            let base_px = cx * 2;
            let base_py = cy * 4;
            let n = cd.density.min(8) as usize;

            for &(col, row) in DENSITY_FILL_ORDER.iter().take(n) {
                canvas.set_pixel(base_px + col, base_py + row, cd.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_grid_uniform_density() {
        let grid = GridData::from_fn(|_, _| 0.5, (0.0, 1.0), (0.0, 1.0), 5, 5);
        let mapper = CoordinateMapper::new(0.0, 1.0, 0.0, 1.0, 10, 20);
        let mut canvas = BrailleCanvas::new(5, 5);
        draw_heatmap(&mut canvas, &grid, &mapper, ColorMapType::Heat);
        // All cells should have some dots set (uniform non-zero value)
        let s = canvas.render_plain();
        let non_blank = s.chars().filter(|&c| c != '\u{2800}' && c != '\n').count();
        assert!(non_blank > 0);
    }
}
