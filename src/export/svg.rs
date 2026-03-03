//! ANSI-to-SVG conversion.

use std::fmt::Write;

/// Converts an ANSI-colored terminal string into an SVG image.
///
/// Wraps the braille characters and ANSI-colored spans inside monospace `<text>` blocks.
///
/// * `ansi_str` - The `Figure::render()` string.
/// * `dark_mode` - If true, uses a dark background and defaults to white text.
///                 If false, uses a light background and defaults to black text.
pub fn to_svg(ansi_str: &str, dark_mode: bool) -> String {
    let (bg_color, default_fg) = if dark_mode {
        ("#1e1e1e", "#d4d4d4")
    } else {
        ("#ffffff", "#333333")
    };

    let lines: Vec<&str> = ansi_str.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().map(|l| strip_ansi(l).chars().count()).max().unwrap_or(0);

    let font_width = 8.0_f64;
    let font_height = 16.0_f64;
    let font_size = 14.0_f64;

    let pad_x = font_width * 2.0;
    let pad_y = font_height;

    let svg_width = cols as f64 * font_width + pad_x * 2.0;
    let svg_height = rows as f64 * font_height + pad_y * 2.0;

    let font_stack = "'DejaVu Sans Mono', 'Fira Code', 'Consolas', 'Liberation Mono', monospace";

    let mut svg = String::new();

    // SVG header
    write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}" viewBox="0 0 {svg_width} {svg_height}">"#,
    )
    .unwrap();

    // Background
    write!(
        svg,
        r#"<rect width="100%" height="100%" fill="{bg_color}" rx="8"/>"#,
    )
    .unwrap();

    // Group with font settings
    write!(
        svg,
        r#"<g font-family="{font_stack}" font-size="{font_size}px" xml:space="preserve">"#,
    )
    .unwrap();

    for (row_idx, line) in lines.iter().enumerate() {
        let y = pad_y + (row_idx as f64 + 1.0) * font_height - (font_height * 0.2);

        write!(svg, r#"<text y="{y}">"#).unwrap();

        // Track the running character offset so each tspan gets an absolute x position.
        let mut char_offset: usize = 0;
        let mut current_color = default_fg;
        let mut current_text = String::new();

        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                if !current_text.is_empty() {
                    write_tspan(&mut svg, current_color, &current_text, pad_x, font_width, char_offset);
                    char_offset += current_text.chars().count();
                    current_text.clear();
                }

                if let Some('[') = chars.peek() {
                    chars.next();
                    let mut code_str = String::new();
                    while let Some(&n) = chars.peek() {
                        if n == 'm' {
                            chars.next();
                            break;
                        }
                        code_str.push(n);
                        chars.next();
                    }

                    current_color = match code_str.as_str() {
                        "0" => default_fg,
                        "31" => if dark_mode { "#f44747" } else { "#cd3131" },
                        "32" => if dark_mode { "#6a9955" } else { "#008000" },
                        "33" => if dark_mode { "#d7ba7d" } else { "#795e26" },
                        "34" => if dark_mode { "#569cd6" } else { "#0000ff" },
                        "35" => if dark_mode { "#c586c0" } else { "#800080" },
                        "36" => if dark_mode { "#4ec9b0" } else { "#008080" },
                        "37" => if dark_mode { "#d4d4d4" } else { "#000000" },
                        _ => current_color,
                    };
                }
            } else {
                current_text.push(c);
            }
        }

        if !current_text.is_empty() {
            write_tspan(&mut svg, current_color, &current_text, pad_x, font_width, char_offset);
        }

        svg.push_str("</text>");
    }

    svg.push_str("</g>");
    svg.push_str("</svg>");

    svg
}

/// Write a `<tspan>` with per-character x positions for pixel-exact placement.
fn write_tspan(svg: &mut String, color: &str, text: &str, pad_x: f64, font_width: f64, char_offset: usize) {
    let n_chars = text.chars().count();
    if n_chars == 0 {
        return;
    }

    // Space-separated list of x coordinates, one per character.
    let x_list: String = (0..n_chars)
        .map(|i| ((pad_x + (char_offset + i) as f64 * font_width) as i64).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    write!(
        svg,
        r#"<tspan x="{x_list}" fill="{color}">{}</tspan>"#,
        escape_xml(text),
    )
    .unwrap();
}

/// Escape characters that are special in XML.
fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Helper to count characters ignoring ANSI sequences
fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            out.push(c);
        }
    }
    out
}
