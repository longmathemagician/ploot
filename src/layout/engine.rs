use crate::canvas::color::TermColor;
use crate::layout::nice_numbers::TickSet;
use crate::layout::text::TextGrid;

/// Configuration for the layout engine.
pub struct LayoutConfig {
    /// Total output width in characters.
    pub total_width: usize,
    /// Total output height in rows.
    pub total_height: usize,
    /// Plot title.
    pub title: Option<String>,
    /// Primary x-axis label.
    pub x_label: Option<String>,
    /// Primary y-axis label.
    pub y_label: Option<String>,
    /// Secondary x-axis label (top).
    pub x2_label: Option<String>,
    /// Secondary y-axis label (right).
    pub y2_label: Option<String>,
    /// Optional secondary x-axis ticks.
    pub x2_ticks: Option<TickSet>,
    /// Optional secondary y-axis ticks.
    pub y2_ticks: Option<TickSet>,
}

/// Computed layout positions.
pub struct Layout {
    /// Row where the title is drawn (if present).
    pub title_row: Option<usize>,
    /// Column offset where the Braille canvas starts.
    pub canvas_col: usize,
    /// Row offset where the Braille canvas starts.
    pub canvas_row: usize,
    /// Character width of the Braille canvas area.
    pub canvas_char_width: usize,
    /// Character height of the Braille canvas area.
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
    /// Row where x2-axis tick labels appear (if secondary x-axis active).
    pub x2_tick_row: Option<usize>,
    /// Row where x2-axis label appears (if present).
    pub x2_label_row: Option<usize>,
    /// Column where y2-axis tick labels start (left edge of y2-tick area).
    pub y2_tick_col_start: Option<usize>,
    /// Width allocated for y2-axis tick labels.
    pub y2_tick_width: Option<usize>,
    /// Column where the y2-axis label appears (if present).
    pub y2_label_col: Option<usize>,
}

/// Computes the layout from a config and tick sets.
pub fn compute_layout(config: &LayoutConfig, x_ticks: &TickSet, y_ticks: &TickSet) -> Layout {
    let has_title = config.title.is_some();
    let has_x_label = config.x_label.is_some();
    let has_y_label = config.y_label.is_some();
    let has_x2 = config.x2_ticks.is_some();
    let has_x2_label = config.x2_label.is_some();
    let has_y2 = config.y2_ticks.is_some();
    let has_y2_label = config.y2_label.is_some();

    // Compute y-tick label width (max width of formatted values)
    let y_tick_width = y_ticks
        .ticks
        .iter()
        .map(|(_, s)| s.len())
        .max()
        .unwrap_or(1)
        .max(1);

    // Compute y2-tick label width
    let y2_tick_width = config
        .y2_ticks
        .as_ref()
        .map(|t| {
            t.ticks
                .iter()
                .map(|(_, s)| s.len())
                .max()
                .unwrap_or(1)
                .max(1)
        })
        .unwrap_or(0);

    // Vertical budget
    let title_rows = if has_title { 1 } else { 0 };
    let x2_label_rows = if has_x2_label { 1 } else { 0 };
    let x2_tick_rows = if has_x2 { 1 } else { 0 };
    let border_rows = 2; // top + bottom border
    let x_tick_rows = 1;
    let x_label_rows = if has_x_label { 1 } else { 0 };

    let vertical_overhead =
        title_rows + x2_label_rows + x2_tick_rows + border_rows + x_tick_rows + x_label_rows;
    let mut canvas_char_height = config.total_height.saturating_sub(vertical_overhead).max(1);

    // Horizontal budget
    let y_label_cols = if has_y_label { 2 } else { 0 }; // "Y " label + space
    let border_cols = 2; // left + right border
    let y_tick_col_width = y_tick_width + 1; // tick labels + space before border
    let y2_tick_col_width = if has_y2 { y2_tick_width + 1 } else { 0 }; // space after border + tick labels
    let y2_label_cols = if has_y2_label { 2 } else { 0 };

    let horizontal_overhead =
        y_label_cols + y_tick_col_width + border_cols + y2_tick_col_width + y2_label_cols;
    let mut canvas_char_width = config
        .total_width
        .saturating_sub(horizontal_overhead)
        .max(1);

    // Shrink canvas to make dimensions tick-aligned so the last tick
    // lands exactly at the border with no trailing dead space.
    let n_y = y_ticks.ticks.len();
    if n_y > 1 {
        canvas_char_height -= (canvas_char_height - 1) % (n_y - 1);
    }
    let n_x = x_ticks.ticks.len();
    if n_x > 1 {
        canvas_char_width -= (canvas_char_width - 1) % (n_x - 1);
    }

    // Compute positions (top to bottom)
    let title_row = if has_title { Some(0) } else { None };
    let mut current_row = title_rows;

    let x2_label_row = if has_x2_label {
        let row = current_row;
        current_row += 1;
        Some(row)
    } else {
        None
    };

    let x2_tick_row = if has_x2 {
        let row = current_row;
        current_row += 1;
        Some(row)
    } else {
        None
    };

    let _top_border_row = current_row;
    current_row += 1; // top border

    let canvas_row_start = current_row;
    current_row += canvas_char_height;

    let bottom_border_row = current_row;
    let _ = bottom_border_row;
    current_row += 1; // bottom border

    let x_tick_row = current_row;
    current_row += 1;

    let x_label_row = if has_x_label {
        let _row = current_row;
        Some(_row)
    } else {
        None
    };

    // Horizontal positions (left to right)
    let y_label_col = if has_y_label { Some(0) } else { None };
    let canvas_col_start = y_label_cols + y_tick_col_width + 1; // +1 for left border

    let y2_tick_col_start = if has_y2 {
        Some(canvas_col_start + canvas_char_width + 1) // +1 for right border
    } else {
        None
    };

    let y2_label_col = if has_y2_label {
        Some(canvas_col_start + canvas_char_width + 1 + y2_tick_col_width)
    } else {
        None
    };

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
        x2_tick_row,
        x2_label_row,
        y2_tick_col_start,
        y2_tick_width: if has_y2 { Some(y2_tick_width) } else { None },
        y2_label_col,
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

    // Y-axis ticks — index-based cell placement matching aligned mapper
    let n_y = y_ticks.ticks.len();
    let cells_per_y = if n_y > 1 {
        ((layout.canvas_char_height - 1) / (n_y - 1)).max(1)
    } else {
        1
    };
    // Ticks are sorted ascending by value. Tick at index i (ascending)
    // maps to cell_row = (N-1-i) * cells_per_y  (Y inversion: smallest
    // value at bottom/high cell row, largest at top/cell 0).
    for (i, (_, label)) in y_ticks.ticks.iter().enumerate() {
        let cell_row = if n_y > 1 {
            (n_y - 1 - i) * cells_per_y
        } else {
            layout.canvas_char_height / 2
        };
        let grid_row = layout.canvas_row + cell_row;
        let label_end = layout.y_tick_col_end;
        grid.put_str_right_aligned(label_end, grid_row, label, TermColor::Default);
        grid.put_char(border_left, grid_row, '┤', TermColor::Default);
    }

    // Y2-axis ticks — on the right border (index-based)
    if let Some(y2_ticks) = &config.y2_ticks
        && let Some(y2_col_start) = layout.y2_tick_col_start
    {
        let n_y2 = y2_ticks.ticks.len();
        let cells_per_y2 = if n_y2 > 1 {
            ((layout.canvas_char_height - 1) / (n_y2 - 1)).max(1)
        } else {
            1
        };
        for (i, (_, label)) in y2_ticks.ticks.iter().enumerate() {
            let cell_row = if n_y2 > 1 {
                (n_y2 - 1 - i) * cells_per_y2
            } else {
                layout.canvas_char_height / 2
            };
            let grid_row = layout.canvas_row + cell_row;
            // Label to the right of the border
            grid.put_str(y2_col_start, grid_row, label, TermColor::Default);
            grid.put_char(border_right, grid_row, '├', TermColor::Default);
        }
    }

    // X-axis ticks — index-based cell placement matching aligned mapper
    let n_x = x_ticks.ticks.len();
    let cells_per_x = if n_x > 1 {
        ((layout.canvas_char_width - 1) / (n_x - 1)).max(1)
    } else {
        1
    };
    for (j, (_, label)) in x_ticks.ticks.iter().enumerate() {
        let cell_col = if n_x > 1 {
            j * cells_per_x
        } else {
            layout.canvas_char_width / 2
        };
        let grid_col = layout.canvas_col + cell_col;
        grid.put_char(grid_col, border_bottom, '┬', TermColor::Default);
        // Center the label below
        let label_len = label.len();
        let label_start = grid_col.saturating_sub(label_len / 2);
        grid.put_str(label_start, layout.x_tick_row, label, TermColor::Default);
    }

    // X2-axis ticks — on the top border (index-based)
    if let Some(x2_ticks) = &config.x2_ticks
        && let Some(x2_tick_row) = layout.x2_tick_row
    {
        let n_x2 = x2_ticks.ticks.len();
        let cells_per_x2 = if n_x2 > 1 {
            ((layout.canvas_char_width - 1) / (n_x2 - 1)).max(1)
        } else {
            1
        };
        for (j, (_, label)) in x2_ticks.ticks.iter().enumerate() {
            let cell_col = if n_x2 > 1 {
                j * cells_per_x2
            } else {
                layout.canvas_char_width / 2
            };
            let grid_col = layout.canvas_col + cell_col;
            grid.put_char(grid_col, border_top, '┴', TermColor::Default);
            let label_len = label.len();
            let label_start = grid_col.saturating_sub(label_len / 2);
            grid.put_str(label_start, x2_tick_row, label, TermColor::Default);
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

    // X2-axis label
    if let Some(row) = layout.x2_label_row
        && let Some(label) = &config.x2_label
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

    // Y2-axis label (written vertically on the right)
    if let Some(col) = layout.y2_label_col
        && let Some(label) = &config.y2_label
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

    fn basic_config(
        width: usize,
        height: usize,
        title: Option<&str>,
        x_label: Option<&str>,
        y_label: Option<&str>,
    ) -> LayoutConfig {
        LayoutConfig {
            total_width: width,
            total_height: height,
            title: title.map(String::from),
            x_label: x_label.map(String::from),
            y_label: y_label.map(String::from),
            x2_label: None,
            y2_label: None,
            x2_ticks: None,
            y2_ticks: None,
        }
    }

    #[test]
    fn layout_dimensions_with_all_decorations() {
        let (x_ticks, y_ticks) = make_ticks();
        let config = basic_config(80, 24, Some("Test"), Some("X"), Some("Y"));
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
        let config = basic_config(40, 12, None, None, None);
        let layout = compute_layout(&config, &x_ticks, &y_ticks);

        // Without decorations, canvas should be larger
        assert!(layout.canvas_char_height > 0);
        assert!(layout.title_row.is_none());
        assert!(layout.x_label_row.is_none());
    }

    #[test]
    fn frame_contains_border_chars() {
        let (x_ticks, y_ticks) = make_ticks();
        let config = basic_config(40, 12, Some("Test"), None, None);
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
        let config = basic_config(40, 12, Some("My Title"), None, None);
        let layout = compute_layout(&config, &x_ticks, &y_ticks);
        let grid = render_frame(&layout, &config, &x_ticks, &y_ticks);
        let s = grid.render_plain();
        assert!(s.contains("My Title"));
    }

    #[test]
    fn layout_with_secondary_axes() {
        let (x_ticks, y_ticks) = make_ticks();
        let y2_ticks = generate_ticks(0.0, 100.0, 5);
        let config = LayoutConfig {
            total_width: 80,
            total_height: 24,
            title: Some("Dual Axis".into()),
            x_label: Some("X".into()),
            y_label: Some("Y1".into()),
            x2_label: None,
            y2_label: Some("Y2".into()),
            x2_ticks: None,
            y2_ticks: Some(y2_ticks),
        };
        let layout = compute_layout(&config, &x_ticks, &y_ticks);

        assert!(layout.y2_tick_col_start.is_some());
        assert!(layout.y2_label_col.is_some());
        assert!(layout.canvas_char_width > 0);
        // Canvas should be narrower with y2 axis
        let config_no_y2 = basic_config(80, 24, Some("Dual Axis"), Some("X"), Some("Y1"));
        let layout_no_y2 = compute_layout(&config_no_y2, &x_ticks, &y_ticks);
        assert!(layout.canvas_char_width < layout_no_y2.canvas_char_width);
    }
}
