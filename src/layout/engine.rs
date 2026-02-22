use crate::canvas::color::TermColor;
use crate::layout::nice_numbers::TickSet;
use crate::layout::text::TextGrid;

/// Configuration for the layout engine.
pub struct LayoutConfig {
    pub total_width: usize,
    pub total_height: usize,
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
}

/// Computed layout positions.
pub struct Layout {
    /// Row where the title is drawn (if present).
    pub title_row: Option<usize>,
    /// Column and row offset where the Braille canvas starts.
    pub canvas_col: usize,
    pub canvas_row: usize,
    /// Character dimensions of the Braille canvas area.
    pub canvas_char_width: usize,
    pub canvas_char_height: usize,
    /// Row where x-axis tick labels appear.
    pub x_tick_row: usize,
    /// Row where the x-axis label appears (if present).
    pub x_label_row: Option<usize>,
    /// Column where y-axis tick labels end (right edge of y-tick area).
    pub y_tick_col_end: usize,
    /// Width allocated for y-axis tick labels.
    pub y_tick_width: usize,
    /// Column where the y-axis label appears (if present).
    pub y_label_col: Option<usize>,
}

/// Computes the layout from a config and tick sets.
pub fn compute_layout(config: &LayoutConfig, _x_ticks: &TickSet, y_ticks: &TickSet) -> Layout {
    let has_title = config.title.is_some();
    let has_x_label = config.x_label.is_some();
    let has_y_label = config.y_label.is_some();

    // Compute y-tick label width (max width of formatted values)
    let y_tick_width = y_ticks
        .ticks
        .iter()
        .map(|(_, s)| s.len())
        .max()
        .unwrap_or(1)
        .max(1);

    // Vertical budget
    let title_rows = if has_title { 1 } else { 0 };
    let border_rows = 2; // top + bottom border
    let x_tick_rows = 1;
    let x_label_rows = if has_x_label { 1 } else { 0 };

    let vertical_overhead = title_rows + border_rows + x_tick_rows + x_label_rows;
    let canvas_char_height = config.total_height.saturating_sub(vertical_overhead).max(1);

    // Horizontal budget
    let y_label_cols = if has_y_label { 2 } else { 0 }; // "Y " label + space
    let border_cols = 2; // left + right border
    let y_tick_col_width = y_tick_width + 1; // tick labels + space before border

    let horizontal_overhead = y_label_cols + y_tick_col_width + border_cols;
    let canvas_char_width = config
        .total_width
        .saturating_sub(horizontal_overhead)
        .max(1);

    // Compute positions
    let title_row = if has_title { Some(0) } else { None };
    let canvas_row_start = title_rows + 1; // +1 for top border
    let canvas_col_start = y_label_cols + y_tick_col_width + 1; // +1 for left border

    let bottom_border_row = canvas_row_start + canvas_char_height;
    let x_tick_row = bottom_border_row + 1;
    let x_label_row = if has_x_label {
        Some(x_tick_row + 1)
    } else {
        None
    };

    let y_label_col = if has_y_label { Some(0) } else { None };

    Layout {
        title_row,
        canvas_col: canvas_col_start,
        canvas_row: canvas_row_start,
        canvas_char_width,
        canvas_char_height,
        x_tick_row,
        x_label_row,
        y_tick_col_end: y_label_cols + y_tick_col_width,
        y_tick_width,
        y_label_col,
    }
}

/// Renders the frame (border, ticks, labels, title) into a TextGrid.
pub fn render_frame(
    layout: &Layout,
    config: &LayoutConfig,
    x_ticks: &TickSet,
    y_ticks: &TickSet,
) -> TextGrid {
    let mut grid = TextGrid::new(config.total_width, config.total_height);

    // Title
    if let Some(row) = layout.title_row
        && let Some(title) = &config.title
    {
        grid.put_str_centered(0, config.total_width, row, title, TermColor::Default);
    }

    let border_top = layout.canvas_row - 1;
    let border_bottom = layout.canvas_row + layout.canvas_char_height;
    let border_left = layout.canvas_col - 1;
    let border_right = layout.canvas_col + layout.canvas_char_width;

    // Top border
    grid.put_char(border_left, border_top, '┌', TermColor::Default);
    for c in (border_left + 1)..border_right {
        grid.put_char(c, border_top, '─', TermColor::Default);
    }
    grid.put_char(border_right, border_top, '┐', TermColor::Default);

    // Bottom border
    grid.put_char(border_left, border_bottom, '└', TermColor::Default);
    for c in (border_left + 1)..border_right {
        grid.put_char(c, border_bottom, '─', TermColor::Default);
    }
    grid.put_char(border_right, border_bottom, '┘', TermColor::Default);

    // Side borders
    for r in layout.canvas_row..border_bottom {
        grid.put_char(border_left, r, '│', TermColor::Default);
        grid.put_char(border_right, r, '│', TermColor::Default);
    }

    // Y-axis ticks — interpolate directly on the left border
    let y_border_span = (border_bottom - border_top) as f64;
    let y_tick_range = y_ticks.max - y_ticks.min;
    for (val, label) in &y_ticks.ticks {
        let t = if y_tick_range.abs() < f64::EPSILON {
            0.5
        } else {
            (val - y_ticks.min) / y_tick_range
        };
        // Higher values at top (lower row numbers)
        let grid_row = border_bottom - (t * y_border_span).round() as usize;
        if grid_row >= border_top && grid_row <= border_bottom {
            let label_end = layout.y_tick_col_end;
            grid.put_str_right_aligned(label_end, grid_row, label, TermColor::Default);
            let tick_char = if grid_row == border_top {
                '┬' // corner: left (tick) + right (top border) + down (left border)
            } else if grid_row == border_bottom {
                '┴' // corner: left (tick) + right (bottom border) + up (left border)
            } else {
                '┤' // mid: up + down (left border) + left (tick)
            };
            grid.put_char(border_left, grid_row, tick_char, TermColor::Default);
        }
    }

    // X-axis ticks — interpolate directly on the bottom border
    let x_border_span = (border_right - border_left) as f64;
    let x_tick_range = x_ticks.max - x_ticks.min;
    for (val, label) in &x_ticks.ticks {
        let t = if x_tick_range.abs() < f64::EPSILON {
            0.5
        } else {
            (val - x_ticks.min) / x_tick_range
        };
        let grid_col = border_left + (t * x_border_span).round() as usize;
        if grid_col >= border_left && grid_col <= border_right {
            // Check if a y-tick already placed a mark at this corner
            let has_y_tick = grid_col == border_left
                && grid.char_at(grid_col, border_bottom) != '└';
            let tick_char = if has_y_tick {
                '┼' // both axes have ticks here
            } else if grid_col == border_left {
                '├' // corner: up (left border) + right (bottom border) + down (tick)
            } else if grid_col == border_right {
                '┤' // corner: up (right border) + left (bottom border) + down (tick)
            } else {
                '┬' // mid: left + right (bottom border) + down (tick)
            };
            grid.put_char(grid_col, border_bottom, tick_char, TermColor::Default);
            // Center the label below
            let label_len = label.len();
            let label_start = grid_col.saturating_sub(label_len / 2);
            grid.put_str(label_start, layout.x_tick_row, label, TermColor::Default);
        }
    }

    // X-axis label
    if let Some(row) = layout.x_label_row
        && let Some(label) = &config.x_label
    {
        grid.put_str_centered(
            layout.canvas_col,
            layout.canvas_char_width,
            row,
            label,
            TermColor::Default,
        );
    }

    // Y-axis label (written vertically in the leftmost column)
    if let Some(col) = layout.y_label_col
        && let Some(label) = &config.y_label
    {
        let chars: Vec<char> = label.chars().collect();
        let start_row =
            layout.canvas_row + layout.canvas_char_height.saturating_sub(chars.len()) / 2;
        for (i, ch) in chars.iter().enumerate() {
            grid.put_char(col, start_row + i, *ch, TermColor::Default);
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::nice_numbers::generate_ticks;

    fn make_ticks() -> (TickSet, TickSet) {
        let x_ticks = generate_ticks(0.0, 10.0, 5);
        let y_ticks = generate_ticks(0.0, 10.0, 5);
        (x_ticks, y_ticks)
    }

    #[test]
    fn layout_dimensions_with_all_decorations() {
        let (x_ticks, y_ticks) = make_ticks();
        let config = LayoutConfig {
            total_width: 80,
            total_height: 24,
            title: Some("Test".to_string()),
            x_label: Some("X".to_string()),
            y_label: Some("Y".to_string()),
        };
        let layout = compute_layout(&config, &x_ticks, &y_ticks);

        // Canvas should be smaller than total
        assert!(layout.canvas_char_width < 80);
        assert!(layout.canvas_char_height < 24);
        assert!(layout.canvas_char_width > 0);
        assert!(layout.canvas_char_height > 0);
    }

    #[test]
    fn layout_dimensions_minimal() {
        let (x_ticks, y_ticks) = make_ticks();
        let config = LayoutConfig {
            total_width: 40,
            total_height: 12,
            title: None,
            x_label: None,
            y_label: None,
        };
        let layout = compute_layout(&config, &x_ticks, &y_ticks);

        // Without decorations, canvas should be larger
        assert!(layout.canvas_char_height > 0);
        assert!(layout.title_row.is_none());
        assert!(layout.x_label_row.is_none());
    }

    #[test]
    fn frame_contains_border_chars() {
        let (x_ticks, y_ticks) = make_ticks();
        let config = LayoutConfig {
            total_width: 40,
            total_height: 12,
            title: Some("Test".to_string()),
            x_label: None,
            y_label: None,
        };
        let layout = compute_layout(&config, &x_ticks, &y_ticks);
        let grid = render_frame(&layout, &config, &x_ticks, &y_ticks);
        let s = grid.render_plain();

        // Corners may become tick chars (┬/┴/├/┤/┼) when ticks land at edges
        assert!(s.contains('┌') || s.contains('┬'));
        assert!(s.contains('┐'));
        assert!(s.contains('├') || s.contains('└') || s.contains('┼'));
        assert!(s.contains('┤') || s.contains('┘'));
        assert!(s.contains('─'));
        assert!(s.contains('│'));
    }

    #[test]
    fn frame_contains_title() {
        let (x_ticks, y_ticks) = make_ticks();
        let config = LayoutConfig {
            total_width: 40,
            total_height: 12,
            title: Some("My Title".to_string()),
            x_label: None,
            y_label: None,
        };
        let layout = compute_layout(&config, &x_ticks, &y_ticks);
        let grid = render_frame(&layout, &config, &x_ticks, &y_ticks);
        let s = grid.render_plain();
        assert!(s.contains("My Title"));
    }
}
