use crate::api::axes::LegendConfig;
use crate::api::options::{DashType, Placement, PlotOption, PointSymbol};
use crate::api::series::SeriesData;
use crate::canvas::PALETTE;
use crate::canvas::color::TermColor;
use crate::layout::text::TextGrid;

/// Entry for a legend row.
struct LegendEntry {
    caption: String,
    color: TermColor,
    is_line: bool,
    is_point: bool,
    dash: DashType,
    symbol: PointSymbol,
}

/// Render legend overlay onto a TextGrid.
pub fn draw_legend(
    grid: &mut TextGrid,
    series: &[SeriesData],
    legend: &LegendConfig,
    canvas_col: usize,
    canvas_row: usize,
    canvas_width: usize,
    canvas_height: usize,
) {
    if !legend.enabled {
        return;
    }

    // Collect entries from series that have captions
    let mut entries = Vec::new();
    for (idx, s) in series.iter().enumerate() {
        let opts = s.options();
        let caption = opts.iter().find_map(|o| {
            if let PlotOption::Caption(c) = o {
                Some(c.clone())
            } else {
                None
            }
        });

        if let Some(caption) = caption {
            let color = opts
                .iter()
                .find_map(|o| {
                    if let PlotOption::Color(c) = o {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .unwrap_or(PALETTE[idx % PALETTE.len()]);

            let dash = opts
                .iter()
                .find_map(|o| {
                    if let PlotOption::LineStyle(d) = o {
                        Some(*d)
                    } else {
                        None
                    }
                })
                .unwrap_or(DashType::Solid);

            let symbol = opts
                .iter()
                .find_map(|o| {
                    if let PlotOption::PointSymbol(p) = o {
                        Some(*p)
                    } else {
                        None
                    }
                })
                .unwrap_or(PointSymbol::Dot);

            let (is_line, is_point) = match s {
                SeriesData::Lines { .. }
                | SeriesData::Boxes { .. }
                | SeriesData::FillBetween { .. } => (true, false),
                SeriesData::Points { .. } => (false, true),
                SeriesData::LinesPoints { .. } => (true, true),
                _ => (true, false),
            };

            entries.push(LegendEntry {
                caption,
                color,
                is_line,
                is_point,
                dash,
                symbol,
            });
        }
    }

    if entries.is_empty() {
        return;
    }

    if legend.reverse {
        entries.reverse();
    }

    // Compute legend dimensions
    let swatch_width = 4; // "───" or "· · " swatch
    let padding = 1;
    let max_caption_len = entries.iter().map(|e| e.caption.len()).max().unwrap_or(0);
    let entry_width = swatch_width + 1 + max_caption_len; // swatch + space + caption

    let (box_width, box_height) = if legend.horizontal {
        let total_w: usize = entries
            .iter()
            .map(|e| swatch_width + 1 + e.caption.len() + 1)
            .sum::<usize>()
            + 1;
        (total_w + 2, 3) // border + content + border
    } else {
        let title_extra = if legend.title.is_some() { 1 } else { 0 };
        (
            entry_width + 2 + padding * 2,
            entries.len() + 2 + title_extra,
        )
    };

    // Position the legend box
    let (start_col, start_row) = match legend.placement {
        Placement::TopRight => {
            let col = canvas_col + canvas_width.saturating_sub(box_width + 1);
            (col, canvas_row)
        }
        Placement::TopLeft => (canvas_col, canvas_row),
        Placement::BottomRight => {
            let col = canvas_col + canvas_width.saturating_sub(box_width + 1);
            let row = canvas_row + canvas_height.saturating_sub(box_height);
            (col, row)
        }
        Placement::BottomLeft => {
            let row = canvas_row + canvas_height.saturating_sub(box_height);
            (canvas_col, row)
        }
    };

    // Draw box border
    grid.put_char(start_col, start_row, '┌', TermColor::Default);
    grid.put_char(
        start_col + box_width - 1,
        start_row,
        '┐',
        TermColor::Default,
    );
    grid.put_char(
        start_col,
        start_row + box_height - 1,
        '└',
        TermColor::Default,
    );
    grid.put_char(
        start_col + box_width - 1,
        start_row + box_height - 1,
        '┘',
        TermColor::Default,
    );
    for c in (start_col + 1)..(start_col + box_width - 1) {
        grid.put_char(c, start_row, '─', TermColor::Default);
        grid.put_char(c, start_row + box_height - 1, '─', TermColor::Default);
    }
    for r in (start_row + 1)..(start_row + box_height - 1) {
        grid.put_char(start_col, r, '│', TermColor::Default);
        grid.put_char(start_col + box_width - 1, r, '│', TermColor::Default);
    }

    // Fill interior with spaces (clear any Braille underneath)
    for r in (start_row + 1)..(start_row + box_height - 1) {
        for c in (start_col + 1)..(start_col + box_width - 1) {
            grid.put_char(c, r, ' ', TermColor::Default);
        }
    }

    // Draw title if present
    let content_start_row = if let Some(title) = &legend.title {
        let title_row = start_row + 1;
        grid.put_str_centered(
            start_col + 1,
            box_width - 2,
            title_row,
            title,
            TermColor::Default,
        );
        title_row + 1
    } else {
        start_row + 1
    };

    // Draw entries
    if legend.horizontal {
        let row = content_start_row;
        let mut col = start_col + 1;
        for entry in &entries {
            draw_swatch(grid, col, row, entry);
            col += swatch_width;
            grid.put_char(col, row, ' ', TermColor::Default);
            col += 1;
            grid.put_str(col, row, &entry.caption, TermColor::Default);
            col += entry.caption.len() + 1;
        }
    } else {
        for (i, entry) in entries.iter().enumerate() {
            let row = content_start_row + i;
            let col = start_col + 1 + padding;
            draw_swatch(grid, col, row, entry);
            grid.put_char(col + swatch_width, row, ' ', TermColor::Default);
            grid.put_str(
                col + swatch_width + 1,
                row,
                &entry.caption,
                TermColor::Default,
            );
        }
    }
}

fn draw_swatch(grid: &mut TextGrid, col: usize, row: usize, entry: &LegendEntry) {
    if entry.is_line {
        let swatch_chars = match entry.dash {
            DashType::Solid => "───",
            DashType::Dash => "╶─╴",
            DashType::Dot => "···",
            DashType::DotDash => "·─·",
            DashType::DotDotDash => "··─",
            DashType::SmallDot => "···",
        };
        let mut c = col;
        for ch in swatch_chars.chars() {
            grid.put_char(c, row, ch, entry.color);
            c += 1;
        }
    } else if entry.is_point {
        let marker = match entry.symbol {
            PointSymbol::Dot => '·',
            PointSymbol::Cross => '+',
            PointSymbol::Circle => 'o',
            PointSymbol::Diamond => '◇',
            PointSymbol::Triangle => '△',
            PointSymbol::Square => '□',
        };
        grid.put_char(col, row, ' ', TermColor::Default);
        grid.put_char(col + 1, row, marker, entry.color);
        grid.put_char(col + 2, row, ' ', TermColor::Default);
    }
}
