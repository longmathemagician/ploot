use crate::api::grid::GridData;
use crate::canvas::BrailleCanvas;
use crate::canvas::color::TermColor;
use crate::canvas::{DashPattern, PALETTE, SOLID};
use crate::transform::CoordinateMapper;
use crate::transform::marching::marching_squares;

/// Draw contour lines from grid data onto the canvas.
pub fn draw_contour(
    canvas: &mut BrailleCanvas,
    grid: &GridData,
    mapper: &CoordinateMapper,
    levels: &[f64],
    color: Option<TermColor>,
    dash: &DashPattern,
) {
    for (i, &level) in levels.iter().enumerate() {
        let line_color = color.unwrap_or(PALETTE[i % PALETTE.len()]);
        let segments = marching_squares(grid, level);

        for seg in &segments {
            let (px0, py0) = mapper.data_to_pixel(seg.x0, seg.y0);
            let (px1, py1) = mapper.data_to_pixel(seg.x1, seg.y1);

            canvas.draw_line(
                px0.round() as i32,
                py0.round() as i32,
                px1.round() as i32,
                py1.round() as i32,
                line_color,
                dash,
            );
        }
    }
}

/// Draw contour lines with default solid dash pattern.
pub fn draw_contour_solid(
    canvas: &mut BrailleCanvas,
    grid: &GridData,
    mapper: &CoordinateMapper,
    levels: &[f64],
    color: Option<TermColor>,
) {
    draw_contour(canvas, grid, mapper, levels, color, &SOLID);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::marching::auto_contour_levels;

    #[test]
    fn tilted_plane_produces_contours() {
        // z = x: contours should be vertical lines
        let grid = GridData::from_fn(|x, _y| x, (0.0, 1.0), (0.0, 1.0), 20, 20);
        let levels = auto_contour_levels(grid.z_min(), grid.z_max(), 5);
        let mapper = CoordinateMapper::new(0.0, 1.0, 0.0, 1.0, 40, 80);
        let mut canvas = BrailleCanvas::new(20, 20);
        draw_contour_solid(&mut canvas, &grid, &mapper, &levels, None);

        let s = canvas.render_plain();
        let non_blank = s.chars().filter(|&c| c != '\u{2800}' && c != '\n').count();
        assert!(non_blank > 0, "contour lines should produce visible output");
    }
}
