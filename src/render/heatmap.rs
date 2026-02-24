use crate::api::grid::GridData;
use crate::canvas::BrailleCanvas;
use crate::canvas::colormap::{ColorMapType, map_color};
use crate::canvas::dither::fill_cell_dithered;
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

            // Map to color+density and fill with ordered dithering
            let cd = map_color(t, colormap);
            fill_cell_dithered(canvas, cx, cy, cd.density, cd.color);
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

    #[test]
    fn no_density_drop_at_band_boundary() {
        // Create a gradient that spans a color band boundary.
        // Heat band boundary is at t=1/7 (~0.143). We'll go from t=0.1 to t=0.2.
        let grid = GridData::from_fn(
            |x, _| x, // z = x, so z ranges linearly
            (0.1, 0.2),
            (0.0, 1.0),
            20,
            2,
        );
        let mapper = CoordinateMapper::new(0.1, 0.2, 0.0, 1.0, 40, 8);
        let mut canvas = BrailleCanvas::new(20, 2);
        draw_heatmap(&mut canvas, &grid, &mapper, ColorMapType::Heat);

        // Count popcount per column and verify it never drops significantly
        // as we move left to right (increasing t).
        let mut prev_total = 0u32;
        let mut drops = 0;
        for cx in 0..20 {
            let mut col_total = 0u32;
            for cy in 0..2 {
                let idx = cy * 20 + cx;
                col_total += canvas.cell_byte(idx).count_ones();
            }
            if col_total + 2 < prev_total {
                drops += 1;
            }
            prev_total = col_total;
        }
        assert!(drops == 0, "density dropped {drops} times across band boundary");
    }
}
